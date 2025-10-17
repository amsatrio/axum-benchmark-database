# schema

## default
```sql
--- CREATE TABLE
CREATE TABLE conditions (
  id varchar(64) not null,
  created_on timestamptz NOT NULL,
  location TEXT NOT NULL,
  temperature DOUBLE PRECISION NULL,
  humidity DOUBLE PRECISION null,
  PRIMARY KEY (id, created_on)
);

--- CREATE INDEXING
CREATE INDEX idx_conditions
ON conditions (id, created_on, location, temperature, humidity);
```

## hypertable timescaledb
```sql
--- CREATE TABLE
CREATE TABLE conditions (
  id varchar(64) not null,
  created_on timestamptz NOT NULL,
  location TEXT NOT NULL,
  temperature DOUBLE PRECISION NULL,
  humidity DOUBLE PRECISION null,
  PRIMARY KEY (id, created_on)
);

--- CREATE HYPERTABLE
SELECT create_hypertable('conditions', 'created_on');

SELECT create_hypertable('conditions', by_range('created_on', INTERVAL '1 second'));

select id,created_on,location,temperature,humidity
from conditions

drop index idx_conditions
```

## partition by date
```sql
--- CREATE TABLE
CREATE TABLE conditions (
  id varchar(64) not null,
  created_on timestamptz NOT NULL,
  location TEXT NOT NULL,
  temperature DOUBLE PRECISION NULL,
  humidity DOUBLE PRECISION null,
  PRIMARY KEY (id, created_on)
) PARTITION BY RANGE (created_on);

--- CREATE INDEXING
CREATE INDEX idx_conditions
ON conditions (id, created_on, location, temperature, humidity);


CREATE TABLE conditions_20251016_1 PARTITION OF conditions FOR VALUES FROM ('2025-10-16 14:15:00+07') TO ('2025-10-16 14:16:00+07');
CREATE TABLE conditions_20251016_2 PARTITION OF conditions FOR VALUES FROM ('2025-10-16 14:16:00+07') TO ('2025-10-16 14:17:00+07');
CREATE TABLE conditions_20251016_3 PARTITION OF conditions FOR VALUES FROM ('2025-10-16 14:17:00+07') TO ('2025-10-16 14:18:00+07');
CREATE TABLE conditions_20251016_4 PARTITION OF conditions FOR VALUES FROM ('2025-10-16 14:18:00+07') TO ('2025-10-16 14:19:00+07');
CREATE TABLE conditions_20251016_5 PARTITION OF conditions FOR VALUES FROM ('2025-10-16 14:19:00+07') TO ('2025-10-16 14:20:00+07');
CREATE TABLE conditions_20251016_6 PARTITION OF conditions FOR VALUES FROM ('2025-10-16 14:20:00+07') TO ('2025-10-16 14:21:00+07');
CREATE TABLE conditions_20251016_7 PARTITION OF conditions FOR VALUES FROM ('2025-10-16 14:21:00+07') TO ('2025-10-16 14:22:00+07');
CREATE TABLE conditions_20251016_8 PARTITION OF conditions FOR VALUES FROM ('2025-10-16 14:22:00+07') TO ('2025-10-16 14:23:00+07');
CREATE TABLE conditions_20251016_9 PARTITION OF conditions FOR VALUES FROM ('2025-10-16 14:23:00+07') TO ('2025-10-16 14:24:00+07');
CREATE TABLE conditions_20251016_10 PARTITION OF conditions FOR VALUES FROM ('2025-10-16 14:24:00+07') TO ('2025-10-16 14:25:00+07');
CREATE TABLE conditions_default PARTITION OF conditions DEFAULT;
```



# result
## hypertable
### create
{"status":200,"message":"Time in milliseconds: 2444,2077,2496,2128,2382,2882,2746,2657,2539,2444 ms","timestamp":"2025-10-17 13:42:34"}
### get
