# PostgreSQL

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



# ----------------------------------------------
# SQL Server
## check partition and indexing size
```sql

SELECT
    t.name AS TableName,
    i.name AS IndexName,
    p.partition_number AS PartitionNumber,
    p.rows AS RowCounts,
    CAST((SUM(a.used_pages) * 8) / 1024.00 AS NUMERIC(18, 2)) AS UsedSpaceMB,
    CAST((SUM(a.total_pages) * 8) / 1024.00 AS NUMERIC(18, 2)) AS AllocatedSpaceMB
FROM
    sys.tables t
INNER JOIN
    sys.indexes i ON t.object_id = i.object_id
INNER JOIN
    sys.partitions p ON i.object_id = p.object_id AND i.index_id = p.index_id
INNER JOIN
    sys.allocation_units a ON p.partition_id = a.container_id
WHERE
    t.name = 'conditions' 
    AND SCHEMA_NAME(t.schema_id) = 'dbo'
GROUP BY
    t.name, i.name, p.partition_number, p.rows
ORDER BY
    i.name, p.partition_number;
    

SELECT
    OBJECT_SCHEMA_NAME(t.object_id) AS SchemaName,
    t.name AS TableName,
    i.name AS IndexName,
    i.index_id AS IndexID,
    CAST(SUM(s.used_page_count) * 8 / 1024.00 AS NUMERIC(18, 2)) AS IndexSizeMB
FROM
    sys.tables t
INNER JOIN
    sys.indexes i ON t.object_id = i.object_id
INNER JOIN
    sys.dm_db_partition_stats s ON i.object_id = s.object_id AND i.index_id = s.index_id
WHERE
    t.is_ms_shipped = 0 
    AND i.index_id > 1  
GROUP BY
    t.object_id, t.name, i.name, i.index_id
ORDER BY
    IndexSizeMB DESC;
```

## disable index and rebuild
```sql
ALTER INDEX ALL ON table_name DISABLE;
ALTER INDEX ALL ON table_name REBUILD;
```

## error when creating index table that has 203 columns
SQL Error [1904] [S0001]: The index 'idx_conditions' on table 'conditions' has 203 columns in the key list. The maximum limit for index key column list is 32.

## 203 columns default no partition no indexing
### insert 100.000 data in table conditions
{
  "status": 200,
  "message": "Time in milliseconds: 73621,70650,82083,83963,83966,89930,90913,94681,96576,95580 ms",
  "timestamp": "2025-11-18 16:06:49"
}
### get 100.000 data in table conditions (the table has 1.000.000 data)
### (select top 100000 * from conditions)
{
  "status": 200,
  "message": "Time in milliseconds: ,145925,157384,161892,165217,170293,163454,166014,175288,166142,162836 ms",
  "timestamp": "2025-11-19 10:39:14"
}
### get 166.670 data in table conditions (the table has 1.000.000 data) 
### (select * from conditions where created_on BETWEEN '2023-01-01' and '2023-12-31')
{
  "status": 200,
  "message": "Time in milliseconds: ,275972,287046,276764,273779,272631,274405,273112,281765,274347,292970 ms",
  "timestamp": "2025-11-19 11:46:44"
}

## 203 columns indexing partition
```sql
CREATE NONCLUSTERED INDEX idx_conditions ON conditions (id,created_on,modified_on,location_1,temperature_1,humidity_1,sensor_numeric_1,sensor_decimal_1);
```
### insert 100.000 data in table conditions
{
  "status": 200,
  "message": "Time in milliseconds: 80574,81147,88738,75546,78845,111868,106014,114315,109800,98969 ms",
  "timestamp": "2025-11-19 14:21:11"
}
### get 166.670 data in table conditions (the table has 1.000.000 data) 
### (select * from conditions where created_on BETWEEN '2023-01-01' and '2023-12-31')
{
  "status": 200,
  "message": "Time in milliseconds: ,267989,281870,280286,259919,260654,271253,264189,261361,264065,265223 ms",
  "timestamp": "2025-11-19 15:10:21"
}