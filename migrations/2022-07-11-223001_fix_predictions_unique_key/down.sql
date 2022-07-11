ALTER TABLE predictions
    DROP CONSTRAINT "predictions_username_name_key",
    ADD CONSTRAINT "predictions_username_key" UNIQUE(username),
    ADD CONSTRAINT "predictions_name_key" UNIQUE(name),

    DROP CONSTRAINT "predictions_username_fkey",
    ADD CONSTRAINT "predictions_username_fkey"
        FOREIGN KEY (username) REFERENCES users(username) 
    ;