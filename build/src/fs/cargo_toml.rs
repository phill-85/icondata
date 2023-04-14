use anyhow::Result;
use heck::ToUpperCamelCase;
use indoc::{formatdoc, indoc};
use std::path::PathBuf;
use tokio::io::AsyncWriteExt;
use tracing::{error, instrument, trace};

use crate::{
    icon::SvgIcon,
    icon_library::IconLibrary,
    main_library::MainLibrary,
    package::{Package, PackageMetadata},
};

const BASE_CARGO_TOML: &str = indoc::indoc!(
    r#"
    # ------------------------------------------------------------------------------------------
    # THIS FILE WAS GENERATED BY THE "BUILD" CRATE.
    # ------------------------------------------------------------------------------------------

"#
);

#[derive(Debug)]
pub(crate) struct CargoToml<T> {
    /// Path to the library's Cargo.toml file.
    pub path: PathBuf,
    pub _phantom: std::marker::PhantomData<T>,
}

impl<T: std::fmt::Debug> CargoToml<T> {
    #[instrument(level = "info")]
    async fn create_file(&self) -> Result<tokio::fs::File> {
        trace!("Creating file.");
        tokio::fs::OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(&self.path)
            .await
            .map_err(|err| {
                error!(?err, "Could not create file.");
                err
            })
            .map_err(Into::into)
    }

    #[instrument(level = "info")]
    pub(crate) async fn reset(&self) -> Result<()> {
        if self.path.exists() {
            trace!("Removing file.");
            tokio::fs::remove_file(&self.path).await?;
        }

        trace!("Writing BASE_CARGO_TOML content.");
        self.create_file()
            .await?
            .write_all(BASE_CARGO_TOML.as_bytes())
            .await
            .map_err(Into::into)
    }

    #[instrument(level = "info", skip_all)]
    async fn append(&self) -> Result<tokio::io::BufWriter<tokio::fs::File>> {
        trace!("Creating file.");
        Ok(tokio::io::BufWriter::new(
            tokio::fs::OpenOptions::new()
                .append(true)
                .open(&self.path)
                .await
                .map_err(|err| {
                    error!(?err, "Could not open file to append data.");
                    err
                })?,
        ))
    }
}

impl CargoToml<MainLibrary> {
    pub(crate) async fn write_cargo_toml(&self, icon_libs: &[IconLibrary]) -> Result<()> {
        self.write_package_section().await?;
        self.write_dependencies_section().await?;
        self.write_features_section(icon_libs).await?;

        Ok(())
    }

    async fn write_package_section(&self) -> Result<()> {
        let package_section = formatdoc! {r#"
                [package]
                name = "icondata"
                version = "0.0.1"
                authors = ["Charles Edward Gagnon"]
                edition = "2021"
                description = "Icons library for the leptos web framework"
                readme = "./README.md"
                repository = "https://github.com/Carlosted/icondata"
                license = "MIT"
                keywords = ["leptos", "icons"]
                categories = ["web-programming"]

                "#};

        let mut file = self.append().await?;

        file.write_all(package_section.as_bytes()).await?;
        file.flush().await.map_err(|err| {
            error!(?err, "Could not flush Cargo.toml file after writing.");
            err
        })?;
        Ok(())
    }

    #[instrument(level = "info")]
    async fn write_dependencies_section(&self) -> Result<()> {
        let mut file = self.append().await?;
        let dependencies = indoc! {r#"
            [dependencies]
            leptos = { version = "0.2.5", default-features = false }
            icondata-core = { path = "../icondata-core" }
            serde = { version = "1", features = ["derive"], optional = true }
            tracing = { version = "0.1", optional = true }

        "#};
        file.write_all(dependencies.as_bytes()).await?;

        for lib in Package::all() {
            file
                // Example: icondata-ai = { path = "../icondata-ai" }
                .write_all(
                    format!(
                        "icondata-{short_name} = {{  path = \"../icondata-{short_name}\", optional = true }}\n",
                        short_name = &lib.meta.short_name
                    )
                    .as_bytes(),
                )
                .await?;
        }

        file.write_all("\n".as_bytes()).await?;
        file.flush().await.map_err(|err| {
            error!(?err, "Could not flush Cargo.toml file after writing.");
            err
        })?;

        Ok(())
    }

    #[instrument(level = "info", skip(icon_libs))]
    async fn write_features_section(&self, icon_libs: &[IconLibrary]) -> Result<()> {
        let mut writer = self.append().await?;

        writer
            .write_all(
                indoc::indoc! {r#"
                [features]
                default = []
                csr = ["leptos/csr", "leptos/tracing", "dep:tracing"]
                hydrate = ["leptos/hydrate", "leptos/tracing", "dep:tracing"]
                ssr = ["leptos/ssr", "leptos/tracing", "dep:tracing"]
                stable = ["leptos/stable", "leptos/tracing", "dep:tracing"]
                serde = ["dep:serde"]

            "#}
                .as_bytes(),
            )
            .await?;

        for package in Package::all() {
            writer
                // Example: Ai = []
                .write_all(
                    format!(
                        "{lib_short_name} = []\n",
                        lib_short_name = &package.meta.short_name.to_upper_camel_case(),
                    )
                    .as_bytes(),
                )
                .await?;
        }

        for lib in icon_libs.iter() {
            for icon in &lib.icons {
                writer
                    // Example: AiPushpinTwotone = ["Ai", "icondata-ai/AiPushpinTwotone"]
                    .write_all(
                        format!(
                            "{feature_name} = [\"{camel_short_name}\", \"icondata-{short_name}/{feature_name}\"]\n",
                            camel_short_name = &lib.package.meta.short_name.to_upper_camel_case(),
                            short_name = &lib.package.meta.short_name,
                            feature_name = icon.feature.name,
                        )
                        .as_bytes(),
                    )
                    .await?;
            }
        }
        writer.flush().await.map_err(|err| {
            error!(?err, "Could not flush Cargo.toml file after writing.");
            err
        })?;

        Ok(())
    }
}

impl CargoToml<IconLibrary> {
    pub(crate) async fn write_cargo_toml(&self, library: &IconLibrary) -> Result<()> {
        self.write_base(&library.package.meta).await?;
        self.write_features(&library.icons).await?;

        Ok(())
    }

    async fn write_base(&self, package_meta: &PackageMetadata) -> Result<()> {
        let base = formatdoc!(
            r#"
            [package]
            name = "icondata-{short_name}"
            version = "0.0.1"
            authors = ["Charles Edward Gagnon"]
            edition = "2021"
            description = "Library providing SVG and corresponding metadata for \"{package_name}\""
            readme = "./README.md"
            repository = "https://github.com/Carlosted/icondata"
            license = "MIT"
            keywords = ["leptos", "icons"]
            categories = ["web-programming"]

            [dependencies]
            icondata-core = {{ path = "../icondata-core" }}
            serde = {{ version = "1", features = ["derive"], optional = true }}

            [features]
            serde = ["dep:serde"]

            "#,
            short_name = package_meta.short_name,
            package_name = package_meta.package_name
        );

        let mut file = self.append().await?;
        file.write_all(base.as_bytes()).await?;
        file.flush().await.map_err(|err| {
            error!(?err, "Could not flush Cargo.toml file after writing.");
            err
        })?;

        Ok(())
    }

    #[instrument(level = "info", skip(icons))]
    async fn write_features(&self, icons: &[SvgIcon]) -> Result<()> {
        trace!(
            num_features = icons.len(),
            "Writing features to Cargo.toml."
        );
        let mut cargo_file = self.append().await?;
        for icon in icons.iter() {
            cargo_file
                .write_all(format!("{} = []\n", &icon.feature.name).as_bytes())
                .await?;
        }
        cargo_file.flush().await.map_err(|err| {
            error!(?err, "Could not flush Cargo.toml file after writing.");
            err
        })?;

        Ok(())
    }
}
