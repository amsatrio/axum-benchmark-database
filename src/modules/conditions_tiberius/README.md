
```sql
--- default
CREATE TABLE conditions (
    id VARCHAR(64) NOT NULL,
    created_on DATETIME2 NOT NULL,
    location NVARCHAR(MAX) NOT NULL,
    temperature FLOAT NULL,
    humidity FLOAT NULL,
    CONSTRAINT PK_conditions PRIMARY KEY CLUSTERED (id, created_on)
);

CREATE NONCLUSTERED INDEX idx_conditions
ON conditions (id, created_on, location, temperature, humidity);
```

```sql
--- partitioned
CREATE PARTITION FUNCTION PF_conditions_yearly (datetime2)
    AS RANGE RIGHT FOR VALUES ('2020-01-01', '2021-01-01', '2022-01-01', '2023-01-01', '2024-01-01', '2025-01-01') ;


CREATE PARTITION SCHEME PS_conditions_yearly
    AS PARTITION PF_conditions_yearly
    ALL TO ('PRIMARY') ;

CREATE TABLE conditions (
    id VARCHAR(64) NOT NULL,
    created_on DATETIME2 NOT NULL,
    location NVARCHAR(MAX) NOT NULL,
    temperature FLOAT NULL,
    humidity FLOAT NULL,
    CONSTRAINT PK_conditions PRIMARY KEY CLUSTERED (id, created_on)
)
ON PS_conditions_yearly (created_on);
```




```sql
ALTER INDEX ALL ON conditions DISABLE;
ALTER INDEX ALL ON conditions REBUILD;
```