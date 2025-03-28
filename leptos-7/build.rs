use std::env;

fn main() {
    #[cfg(target_os = "windows")]
    {
        println!("cargo:rustc-cfg=blog_test");
    }
    let test_db_url = "";
    let test_blog_content = "./posts/blog";
    match env::var("BLOG_TEST_STATUS") {
        Ok(val) if val == "true" => {
            println!("cargo:rustc-cfg=blog_test");
            println!("cargo:rustc-env=DATABASE_URL={}", test_db_url);
            println!("cargo:rustc-env=BLOG_FOLDER={}", test_blog_content);
        }
        Ok(val) => {
            println!("Environment variable BLOG_TEST_STATUS is set to '{}'.", val);
        }
        Err(_) => {
            println!("Environment variable BLOG_TEST_STATUS is not set.");
        }
    }
}
