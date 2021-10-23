-- Your SQL goes here

CREATE TABLE "items"(
   "id" integer not null PRIMARY KEY autoincrement,
   "name" varchar not null,
   "price" integer not null,
   "created_at" TIMESTAMP not null,
   "updated_at" TIMESTAMP not null
);