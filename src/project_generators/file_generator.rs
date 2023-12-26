use std::{path::Path, io};

use std::fs::{create_dir_all, self};

pub trait FileGenerator {
    fn generate_file_content(&self, static_template_content: String, dynamic_template: String) -> String {
        static_template_content + "\n" + dynamic_template.as_str()
    }

    fn generate_file(&self, static_template_content: String, dynamic_template: String, out_path: &str) -> io::Result<()> {
        let file_content = self.generate_file_content(static_template_content, dynamic_template);
        let file_path = Path::new(out_path);
        create_dir_all(file_path.parent().unwrap())?;
        std::fs::write(file_path, file_content).map_err(|e| {println!("Error: {:?}", e); e})?;
        Ok(())
    }

    fn copy_dir_all(&self, src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
        fs::create_dir_all(&dst)?;
        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let ty = entry.file_type()?;
            if ty.is_dir() {
                self.copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
            } else {
                fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
            }
        }
        Ok(())
    }
}