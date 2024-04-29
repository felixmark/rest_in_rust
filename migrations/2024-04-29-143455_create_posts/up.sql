-- Your SQL goes here
CREATE TABLE "notes"(
	"id" UUID NOT NULL PRIMARY KEY,
	"timestamp" TIMESTAMP NOT NULL,
	"content" TEXT NOT NULL
);

