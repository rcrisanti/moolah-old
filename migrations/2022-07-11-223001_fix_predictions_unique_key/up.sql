ALTER TABLE predictions
    -- remove 2 separate unique constraints & make 1 combined unique constraint
    DROP CONSTRAINT "predictions_username_key",
    DROP CONSTRAINT "predictions_name_key",
    ADD CONSTRAINT "predictions_username_name_key" UNIQUE(username, name),
    
    -- update foregin key to cascade
    DROP CONSTRAINT "predictions_username_fkey",
    ADD CONSTRAINT "predictions_username_fkey"
        FOREIGN KEY (username) REFERENCES users(username) 
            ON UPDATE CASCADE 
            ON DELETE CASCADE
    ;