-- Add up migration script here

-- Nomer table
CREATE TABLE IF NOT EXISTS `Nomer` (
    `Id` INTEGER NOT NULL PRIMARY KEY AUTO_INCREMENT,
    `DisplayName` VARCHAR(255) NOT NULL UNIQUE,
    `Email` VARCHAR(255) NOT NULL UNIQUE,
    `PasswordHash` VARCHAR(255) NOT NULL UNIQUE
);