use std::{path::Path, io};

use std::fs::create_dir_all;

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
}