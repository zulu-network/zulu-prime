ALTER TABLE snapshots
    ADD COLUMN version INT NOT NULL DEFAULT 0;