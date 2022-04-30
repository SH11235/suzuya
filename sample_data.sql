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

INSERT INTO items (title, name, product_code, release_date, arrival_date, reservation_start_date, reservation_deadline, order_date, sku, illust_status, design_status, last_updated, retail_price, catalog_status, announcement_status, remarks, maker_id, pic_id, double_check_person_id) VALUES
    ('title', 'name', null, null, null, null, null, null, null, '未着手(sample)', '未着手(sample)', '2022-02-22T22:22:22+09:00', null, '未着手(sample)', '未着手(sample)', null, 1, 1, null)
    ,('title', 'name', 'A123456', '2022-12-22T22:22:22+09:00', '2022-02-22T22:22:22+09:00', '2022-02-22T22:22:22+09:00', '2022-02-22T22:22:22+09:00', '2022-02-22T22:22:22+09:00', 89, 'イラスト納品待ち', '入稿データ作成中', '2022-02-28T22:22:22+09:00', 5000, '作成済み', '作成済み', 'ここには備考が入ります。文字数文字数文字数文字数文字数文字数文字数', 1, 1, 1)
;
