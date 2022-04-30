INSERT INTO makers (code_name) VALUES
    ('AA'),
    ('BB'),
    ('CC'),
    ('DD'),
    ('EE'),
    ('FF'),
    ('GG'),
    ('HH'),
    ('II'),
    ('JJ')
;

INSERT INTO users (name, deleted, created_at, updated_at) VALUES
    ('name1', 'false', '2000-01-01T00:00:00+09:00', '2000-01-01T00:00:00+09:00'),
    ('name2', 'false', '2022-02-22T22:22:22+09:00', '2022-12-22T22:22:22+09:00')
;

INSERT INTO items (title, name, product_code,arrival_date, reservation_start_date, reservation_deadline, order_date, sku, illust_status, design_status, last_updated, retail_price, catalog_status, announcement_status, remarks, maker_id, pic_id, double_check_person_id) VALUES
    ('title', 'name', null, null, null, null, null, null, '未着手(白)', '未着手(白)', '2022-02-22T22:22:22+09:00', null, '未着手(白)', '未着手(白)', null, 1, 1, null)
;
