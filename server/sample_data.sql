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

INSERT INTO items (
    release_date,
    reservation_start_date,
    reservation_deadline,
    order_date,
    title,
    project_type,
    last_updated,
    name,
    product_code,
    sku,
    illust_status,
    pic_illust_id,
    design_status,
    pic_design_id,
    maker_id,
    retail_price,
    double_check_person_id,
    catalog_status,
    announcement_status,
    remarks
) VALUES
    (
        null,
        null,
        null,
        null,
        '日付未定サンプル_20220504022303',
        'デフォルト',
        '2022-05-04T04:24:22+09:00',
        'アイテム名サンプル',
        null,
        null,
        '未着手',
        null,
        '未着手',
        null,
        null,
        null,
        null,
        '未着手',
        '未着手',
        null
     ),
     (
        '2022-08-04T04:24:22+09:00',
        '2022-07-04T04:24:22+09:00',
        '2022-07-11T04:24:22+09:00',
        '2022-07-18T04:24:22+09:00',
        'アイテムタイトル_20220504044723',
        'A案件',
        '2022-05-04T04:47:23+09:00',
        'アイテム名',
        'ABC-11235',
        5,
        'イラスト納品待ち',
        1,
        '素材提供待ち',
        2,
        1,
        500,
        2,
        '未着手',
        '未着手',
        '備考'
     )
;
