// SPDX-License-Identifier: Apache-2.0
use glib::prelude::*;
use std::hint::black_box;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    ffi_conversion_contracts()?;
    slice_ownership_contract();
    assert!(glib::LogLevel::try_from(0x8000_0000).is_err());
    assert!(glib::LogLevel::try_from(0).is_err());
    for level in [
        glib::LogLevel::Error,
        glib::LogLevel::Critical,
        glib::LogLevel::Warning,
        glib::LogLevel::Message,
        glib::LogLevel::Info,
        glib::LogLevel::Debug,
    ] {
        let flags = glib::translate::IntoGlib::into_glib(level);
        assert_eq!(glib::LogLevel::try_from(flags), Ok(level));
    }
    for _ in 0..1000 {
        let values = ["first", "", "中文", "last"];
        let variant = black_box(values.to_variant());
        assert_eq!(variant.array_iter_str()?.collect::<Vec<_>>(), values);
        assert_eq!(
            variant.array_iter_str()?.rev().collect::<Vec<_>>(),
            ["last", "中文", "", "first"]
        );
        let mut mixed = variant.array_iter_str()?;
        assert_eq!(mixed.next(), Some("first"));
        assert_eq!(mixed.next_back(), Some("last"));
        assert_eq!(mixed.nth(1), Some("中文"));
        assert_eq!(mixed.next_back(), None);
        assert_eq!(variant.array_iter_str()?.nth_back(1), Some("中文"));
        assert_eq!(variant.array_iter_str()?.last(), Some("last"));
        assert_eq!(variant.array_iter_str()?.nth(usize::MAX), None);
        let empty = Vec::<String>::new().to_variant();
        assert_eq!(empty.array_iter_str()?.next(), None);
    }
    std::io::Write::write_all(
        &mut std::io::stdout(),
        b"Optimized GLib iterator, typed FFI conversion and slice ownership regression passed\n",
    )?;
    Ok(())
}

fn ffi_conversion_contracts() -> Result<(), Box<dyn std::error::Error>> {
    let (_, charset) = glib::charset();
    assert!(charset.is_some_and(|value| !value.is_empty()));
    let (_, filename_charsets) = glib::filename_charsets();
    assert!(!filename_charsets.is_empty());
    assert!(filename_charsets.iter().all(|value| !value.is_empty()));
    let (scheme, userinfo, host, port, path, query, fragment) = glib::Uri::split(
        "https://example.org:8443/path?q=1#part",
        glib::UriFlags::empty(),
    )?;
    assert_eq!(scheme.as_ref().map(|value| value.as_str()), Some("https"));
    assert!(userinfo.is_none());
    assert_eq!(
        host.as_ref().map(|value| value.as_str()),
        Some("example.org")
    );
    assert_eq!(port, 8443);
    assert_eq!(path.as_str(), "/path");
    assert_eq!(query.as_ref().map(|value| value.as_str()), Some("q=1"));
    assert_eq!(fragment.as_ref().map(|value| value.as_str()), Some("part"));
    let (scheme, userinfo, host, port, path, query, fragment) =
        glib::Uri::split("relative", glib::UriFlags::empty())?;
    assert!(scheme.is_none() && userinfo.is_none() && host.is_none());
    assert_eq!(port, -1);
    assert_eq!(path.as_str(), "relative");
    assert!(query.is_none() && fragment.is_none());
    let (_, _, password, auth, _, _, _, _, _) = glib::Uri::split_with_user(
        "https://name:fixture;meta@example.org/path",
        glib::UriFlags::HAS_PASSWORD | glib::UriFlags::HAS_AUTH_PARAMS,
    )?;
    assert!(password.as_ref().map(|value| value.as_str()) == Some("fixture"));
    assert!(auth.as_ref().map(|value| value.as_str()) == Some("meta"));
    let (scheme, host, port) =
        glib::Uri::split_network("https://example.org:8443/path", glib::UriFlags::empty())?;
    assert_eq!(scheme.as_ref().map(|value| value.as_str()), Some("https"));
    assert_eq!(
        host.as_ref().map(|value| value.as_str()),
        Some("example.org")
    );
    assert_eq!(port, 8443);
    let (scheme, user, password, auth, host, port, path, query, fragment) =
        glib::Uri::split_with_user("https://name@example.org/path", glib::UriFlags::empty())?;
    assert_eq!(scheme.as_ref().map(|value| value.as_str()), Some("https"));
    assert_eq!(user.as_ref().map(|value| value.as_str()), Some("name"));
    assert!(password.is_none() && auth.is_none());
    assert_eq!(
        host.as_ref().map(|value| value.as_str()),
        Some("example.org")
    );
    assert_eq!(port, -1);
    assert_eq!(path.as_str(), "/path");
    assert!(query.is_none() && fragment.is_none());
    let input = "Unicode 中文".as_bytes();
    let (converted, bytes_read) = glib::convert(input, "UTF-8", "UTF-8")?;
    assert_eq!(&*converted, input);
    assert_eq!(bytes_read, input.len());
    assert!(matches!(
        glib::convert(&[0xff], "UTF-8", "UTF-8"),
        Err(glib::CvtError::IllegalSequence { offset: 0, .. })
    ));
    let key_file = glib::KeyFile::new();
    assert!(key_file
        .load_from_data("invalid data", glib::KeyFileFlags::empty())
        .is_err());
    key_file.load_from_data(
        "[group]\nname=中文\nnumber=42\n",
        glib::KeyFileFlags::empty(),
    )?;
    assert_eq!(key_file.string("group", "name")?.as_str(), "中文");
    assert_eq!(key_file.integer("group", "number")?, 42);
    assert!(key_file.string("group", "missing").is_err());
    assert!(key_file.integer("group", "name").is_err());
    assert!(glib::shell_parse_argv("'unterminated").is_err());
    assert!(glib::Uri::parse("not a URI", glib::UriFlags::empty()).is_err());
    assert!(glib::Uri::parse("https://example.org/path", glib::UriFlags::empty()).is_ok());
    assert!(glib::Variant::parse(None, "not a variant").is_err());
    assert_eq!(
        glib::Variant::parse(None, "'中文'")?
            .get::<String>()
            .as_deref(),
        Some("中文")
    );
    assert!(glib::uuid_string_is_valid(
        "f81d4fae-7dec-11d0-a765-00a0c91e6bf6"
    ));
    assert!(!glib::uuid_string_is_valid("not-a-uuid"));
    if let Some(charset) = glib::console_charset() {
        assert!(!charset.is_empty());
    }
    let path = std::path::PathBuf::from("/tmp/vextessera unicode 中文.txt");
    let uri = glib::filename_to_uri(&path, None)?;
    let (round_trip, hostname) = glib::filename_from_uri(&uri)?;
    assert_eq!(round_trip, path);
    assert!(hostname.is_none());
    assert!(glib::filename_from_uri("not-a-file-uri").is_err());
    assert!(glib::filename_to_uri("relative-path", None).is_err());
    let missing = std::env::temp_dir().join(format!(
        "vextessera-glib-regression-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_nanos()
    ));
    assert!(!missing.exists());
    let file = missing.join("uncreated-parent").join("test.txt");
    assert!(glib::file_set_contents(&file, b"test").is_err());
    assert!(glib::file_set_contents_full(
        &file,
        b"test",
        glib::FileSetContentsFlags::empty(),
        0o600
    )
    .is_err());
    assert!(!missing.exists());
    Ok(())
}

fn slice_ownership_contract() {
    let kept = glib::Object::new::<glib::Object>();
    let removed = glib::Object::new::<glib::Object>();
    let removed_weak = removed.downgrade();
    let mut slice = glib::collections::PtrSlice::from([kept.clone(), removed]);
    assert_eq!(kept.ref_count(), 2);
    slice.truncate(1);
    assert_eq!(slice.len(), 1);
    assert!(removed_weak.upgrade().is_none());
    assert_eq!(kept.ref_count(), 2);
    slice.truncate(0);
    assert!(slice.is_empty());
    assert_eq!(kept.ref_count(), 1);
    slice.truncate(0);
    assert_eq!(kept.ref_count(), 1);
}
