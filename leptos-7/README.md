## DB

```
sudo -u postgres psql -c "CREATE USER blog_user WITH PASSWORD 'yo';"
sudo -u postgres psql -c "ALTER USER blog_user CREATEDB;"
sudo -u postgres psql -c "CREATE DATABASE tadgh_blog_db OWNER blog_user;"
psql --username=postgres  postgres < ./shared/blog-db/db_backup.txt
CREATE TABLE tadgh_blog.blog_post_ips (
    id SERIAL PRIMARY KEY,
    blog_post_id INTEGER NOT NULL REFERENCES tadgh_blog.blog_posts(id) ON DELETE CASCADE,
    ip_address INET NOT NULL
);
GRANT USAGE, SELECT ON SEQUENCE tadgh_blog.blog_post_ips_id_seq TO your_user;
GRANT INSERT ON TABLE tadgh_blog.blog_post_ips TO blog_user;

```

## Styles

```
npx tailwindcss -i ./style/tailwind.css -o ./style/output.css --minify
```

## ENV

add to bashrc

```
export BLOG_TEST_STATUS=true
export DATABASE_URL=""
export BLOG_FOLDER="./posts/blog"
```
