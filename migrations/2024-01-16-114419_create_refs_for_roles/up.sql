-- Your SQL goes here
ALTER TABLE "user_roles" ADD FOREIGN KEY ("user_username") REFERENCES "users" ("username");
ALTER TABLE "user_roles" ADD FOREIGN KEY ("role_id") REFERENCES "roles" ("id");
ALTER TABLE "user_roles" ADD CONSTRAINT "unique_user_role" UNIQUE ("user_username", "role_id");

