CREATE TABLE usdt_btc (
 id INT PRIMARY KEY GENERATED BY DEFAULT AS IDENTITY,
 date TIMESTAMP NOT NULL,
 open DOUBLE PRECISION NOT NULL,
 high DOUBLE PRECISION NOT NULL,
 low DOUBLE PRECISION NOT NULL,
 close DOUBLE PRECISION NOT NULL,
 volume DOUBLE PRECISION NOT NULL
);