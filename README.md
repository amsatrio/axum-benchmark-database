# timescaledb
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

# partition by hash
```sql
--- CREATE TABLE
CREATE TABLE conditions (
  id varchar(64) not null,
  created_on timestamptz NOT NULL,
  location TEXT NOT NULL,
  temperature DOUBLE PRECISION NULL,
  humidity DOUBLE PRECISION null,
  PRIMARY KEY (id, created_on) 
) PARTITION BY hash (id);

--- CREATE INDEXING
CREATE INDEX idx_conditions
ON conditions (id, created_on, location, temperature, humidity);


CREATE TABLE conditions_1 PARTITION OF conditions
    FOR VALUES WITH (MODULUS 5, REMAINDER 0);
CREATE TABLE conditions_2 PARTITION OF conditions
    FOR VALUES WITH (MODULUS 5, REMAINDER 1);
CREATE TABLE conditions_3 PARTITION OF conditions
    FOR VALUES WITH (MODULUS 5, REMAINDER 2);
CREATE TABLE conditions_4 PARTITION OF conditions
    FOR VALUES WITH (MODULUS 5, REMAINDER 3);
CREATE TABLE conditions_5 PARTITION OF conditions
    FOR VALUES WITH (MODULUS 5, REMAINDER 4);

```


# partition by date
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


CREATE TABLE conditions_20251003_1 PARTITION OF conditions FOR VALUES FROM ('2025-10-03 13:45:00+07') TO ('2025-10-03 13:46:00+07');
CREATE TABLE conditions_20251003_2 PARTITION OF conditions FOR VALUES FROM ('2025-10-03 13:46:00+07') TO ('2025-10-03 13:47:00+07');
CREATE TABLE conditions_20251003_3 PARTITION OF conditions FOR VALUES FROM ('2025-10-03 13:47:00+07') TO ('2025-10-03 13:48:00+07');
CREATE TABLE conditions_20251003_4 PARTITION OF conditions FOR VALUES FROM ('2025-10-03 13:48:00+07') TO ('2025-10-03 13:49:00+07');
CREATE TABLE conditions_20251003_5 PARTITION OF conditions FOR VALUES FROM ('2025-10-03 13:49:00+07') TO ('2025-10-03 13:50:00+07');
CREATE TABLE conditions_20251003_6 PARTITION OF conditions FOR VALUES FROM ('2025-10-03 13:50:00+07') TO ('2025-10-03 13:51:00+07');
CREATE TABLE conditions_20251003_7 PARTITION OF conditions FOR VALUES FROM ('2025-10-03 13:51:00+07') TO ('2025-10-03 13:52:00+07');
CREATE TABLE conditions_20251003_8 PARTITION OF conditions FOR VALUES FROM ('2025-10-03 13:52:00+07') TO ('2025-10-03 13:53:00+07');
CREATE TABLE conditions_20251003_9 PARTITION OF conditions FOR VALUES FROM ('2025-10-03 13:53:00+07') TO ('2025-10-03 13:54:00+07');
CREATE TABLE conditions_20251003_10 PARTITION OF conditions FOR VALUES FROM ('2025-10-03 13:54:00+07') TO ('2025-10-03 13:55:00+07');
CREATE TABLE conditions_default PARTITION OF conditions DEFAULT;
```


# result
postgresql																
	data 1 juta															
																
			(ms)	1	2	3	4	5	6	7	8	9	10	avg	avg (second)	
		default	insert	37529	43419	46771	49485	67787	59707	59982	61447	53204	59626	53895,7	53,8957	
			get all	3102	2992	3639	4315	4272	4141	3979	4323	4260	4284	3930,7	3,9307	
		partitioning by hash id	insert	38386	41817	42571	46807	48314	63537	67882	70853	67993	62140	55030	55,03	
			get all	3928	5087	4909	4816	4641	4267	4254	4304	4173	4368	4474,7	4,4747	
		hypertable	insert	31741	33301	33832	34618	33653	36269	36963	44878	41651	40080	36698,6	36,6986	
			get all	3352	3033	2951	2869	3086	2986	2977	3022	3031	3055	3036,2	3,0362	
		partitioning by date	insert	34319	35017	34790	36784	39640	46389	44864	58563	54942	55676	44098,4	44,0984	
			get all	4221	3293	2926	3147	3279	3503	3770	4365	4060	5882	3844,6	3,8446	
																
				1	2	3	4	5	6	7	8	9	10	avg	avg (second)	%
		insert	default	37529	43419	46771	49485	67787	59707	59982	61447	53204	59626	53895,7	53,8957	68,09188859
			partitioning by hash id	38386	41817	42571	46807	48314	63537	67882	70853	67993	62140	55030	55,03	66,68835181
			partitioning by date	34319	35017	34790	36784	39640	46389	44864	58563	54942	55676	44098,4	44,0984	83,21979936
			hypertable	31741	33301	33832	34618	33653	36269	36963	44878	41651	40080	36698,6	36,6986	100
				1	2	3	4	5	6	7	8	9	10	avg	avg (second)	%
		get all	default	3102	2992	3639	4315	4272	4141	3979	4323	4260	4284	3930,7	3,9307	77,24323912
			partitioning by hash id	3928	5087	4909	4816	4641	4267	4254	4304	4173	4368	4474,7	4,4747	67,85259347
			partitioning by date	4221	3293	2926	3147	3279	3503	3770	4365	4060	5882	3844,6	3,8446	78,97310513
			hypertable	3352	3033	2951	2869	3086	2986	2977	3022	3031	3055	3036,2	3,0362	100

