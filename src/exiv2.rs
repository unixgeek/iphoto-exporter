use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

pub(crate) struct ScriptGenerator {
    writer: BufWriter<File>,
}

impl ScriptGenerator {
    pub(crate) fn new(script_path: &PathBuf) -> Self {
        let mut script = PathBuf::from(script_path);
        script.push("set-image-comments.sh");

        let mut writer = BufWriter::new(File::create(script).expect("opening exiv2 script file"));
        writeln!(writer, "#!/bin/sh -e").unwrap();

        ScriptGenerator { writer }
    }

    pub(crate) fn add_comment(&mut self, comment: &str, file: &str) {
        // Replace " with \".
        let escaped_comment = comment.to_string().replace('"', r#"\""#);

        writeln!(
            self.writer,
            r#"exiv2 -q -M "set Exif.Image.ImageDescription Ascii {}" "{}""#,
            escaped_comment, file
        )
        .unwrap();

        let mut comment_as_byte: String = String::new();
        comment.as_bytes().iter().for_each(|byte| {
            comment_as_byte.push_str(&byte.to_string());
            comment_as_byte.push(' ');
            comment_as_byte.push('0');
            comment_as_byte.push(' ');
        });
        writeln!(
            self.writer,
            r#"exiv2 -q -M "set Exif.Image.XPComment Byte {}" "{}""#,
            comment_as_byte, file
        )
        .unwrap();

        writeln!(
            self.writer,
            r#"exiv2 -q -M "set Iptc.Application2.Caption String {}" "{}""#,
            escaped_comment, file
        )
        .unwrap();

        writeln!(
            self.writer,
            r#"exiv2 -q -M "set Xmp.dc.description LangAlt lang=en-US {}" "{}""#,
            escaped_comment, file
        )
        .unwrap();

        self.writer.flush().expect("flushing script")
    }
}
