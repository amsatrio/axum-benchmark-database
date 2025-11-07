
```sql
CREATE TABLE conditions (
    id VARCHAR(64) NOT NULL,
    created_on DATETIMEOFFSET NOT NULL,
    location NVARCHAR(MAX) NOT NULL,
    temperature FLOAT NULL,
    humidity FLOAT NULL,
    CONSTRAINT PK_conditions PRIMARY KEY CLUSTERED (id, created_on)
);

CREATE NONCLUSTERED INDEX idx_conditions
ON conditions (id, created_on, location, temperature, humidity);

-- Partition Function: Partitions data by the start of each year
CREATE PARTITION FUNCTION PF_conditions_yearly (DATETIMEOFFSET(7))
AS RANGE RIGHT
FOR VALUES (
    '2023-01-01T00:00:00.000 +00:00', -- Boundary 1 (Data before this goes to Partition 1)
    '2024-01-01T00:00:00.000 +00:00', -- Boundary 2
    '2025-01-01T00:00:00.000 +00:00'  -- Boundary 3
    -- This creates 4 partitions: < 2023, 2023, 2024, >= 2025
);
GO

-- Partition Scheme: Maps the partitions to filegroups
CREATE PARTITION SCHEME PS_conditions_yearly
AS PARTITION PF_conditions_yearly
ALL TO ([PRIMARY]);
GO

CREATE TABLE conditions (
    id VARCHAR(64) NOT NULL,
    created_on DATETIMEOFFSET(7) NOT NULL, -- DATETIMEOFFSET(7) is commonly used
    location NVARCHAR(MAX) NOT NULL,
    temperature FLOAT NULL,
    humidity FLOAT NULL
)
-- CLUSTERED PRIMARY KEY MUST be built on the partition scheme, 
-- and the partition column (created_on) must be part of the key.
ON PS_conditions_yearly (created_on);
GO

-- Create the CLUSTERED PRIMARY KEY (This is what makes the table partitioned)
ALTER TABLE conditions
ADD CONSTRAINT PK_conditions_partitioned PRIMARY KEY CLUSTERED (id, created_on)
ON PS_conditions_yearly (created_on);
GO
```






CREATE TABLE db_penampungan_oracle.dbo.MD_PENAMPUNGAN_partition (
	id_md_penampungan int NOT NULL,
	kode_file nvarchar(100)  NOT NULL,
	tgl_input datetime NOT NULL,
	tgl_proses date NULL,
	nominal decimal(24,8) NOT NULL,
	tgl_jurnal date NULL,
	id_syariah nvarchar(32)  NOT NULL,
	id_dc_sektor_lbu nchar(2)  NULL,
	kode_wilayah char(1)  NULL,
	kode_pos int NULL,
	tipe_kantor int NULL,
	kode_lob char(1)  NULL,
	grouping_produk_erp int NULL,
	sub_kode_file varchar(20)  NULL,
	flag_koreksi int NULL,
	batch_id varchar(50)  NULL
);