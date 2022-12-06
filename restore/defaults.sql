INSERT INTO employee_role (name) VALUES ('Управляющий отелем');
INSERT INTO employee_role (name) VALUES ('Администратор');

INSERT INTO person (
    name, surname, last_name, phone_number, email
) VALUES (
    'Александр',
    'Александров',
    'Александрович',
    '73512662154',
    'alexander@gmail.com'
);

INSERT INTO person (
    name, surname, last_name, phone_number, email
) VALUES (
    'Максим',
    'Максимов',
    'Максимович',
    '78662983742',
    'maksim@gmail.com'
);

INSERT INTO employee (
    person_id, role_id
) VALUES (
    1,
    1
);

INSERT INTO employee (
    person_id, role_id
) VALUES (
    2,
    1
);

INSERT INTO hotel (
    name, city_id, owner_id, description
) VALUES (
    'Palazzo 4*',
    1341,
    1,
    'Отличный номер люкс за 4 т.р лепнина на потолке, просторный двухместный номер с джакузи, есть ресторан этажом ниже. Вежливый персонал, в номере чисто и уютно, есть даже халаты.'
);

INSERT INTO hotel (
    name, city_id, owner_id, description
) VALUES (
    'Valo Hotel City 3*',
    173,
    2,
    'Хороший отель, уютные номера, все чистенько и опрятно. Завтраки хорошие, отличный фитнес центр и бассейн.'
);

INSERT INTO internal.database_user (
    email, password_hash
) VALUES (
    'primitive_email@not.even.valid',
    '$2b$12$agd/YFAsOd9Fsf.H82VnWuvhWGmf1uiUOAef4f1Vanpwo/x/6xe22'
);

INSERT INTO client_type (
    name
) VALUES (
    'Физическое лицо'
);

INSERT INTO client_type (
    name
) VALUES (
    'Юридическое лицо'
);

INSERT INTO person (
    name, surname, last_name, phone_number, email
) VALUES (
    'Дмитрий',
    'Дмитриев',
    'Дмитриевич',
    '71642983548',
    'dmitry@gmail.com'
);

INSERT INTO person (
    name, surname, last_name, phone_number, email
) VALUES (
    'Алексей',
    'Алексеев',
    'Алексеевич',
    '75552386848',
    'alexey@gmail.com'
);

INSERT INTO client (
    type_id, person_id
) VALUES (
    1,
    3
);

INSERT INTO client (
    type_id, person_id
) VALUES (
    2,
    4
);


INSERT INTO tour_feeding_type (
    name
) VALUES (
    'без питания'
);

INSERT INTO tour_feeding_type (
    name
) VALUES (
    'с завтраком'
);

INSERT INTO tour_feeding_type (
    name
) VALUES (
    '3-х разовое'
);


INSERT INTO tour (
    hotel_id,
    arrival_date,
    departure_date,
    feeding_type_id,
    cost,
    description
) VALUES (
    1,
    '2022-12-27',
    '2023-1-8',
    3,
    10005,
    'Расположение Путешествовать экономно — легко. Отель «Гостиница Ковров» расположен в Коврове. Этот отель находится в самом центре города. Перед сном есть возможность прогуляться вдоль главных достопримечательностей. Рядом с отелем — Борисоглебский собор, Церковь Бориса и Глеба и Свято-Васильевский Монастырь.В отеле Время вспомнить о хлебе насущном! Для гостей работает ресторан. Кафе отеля — удобное место для перекуса.'
);

INSERT INTO tour (
    hotel_id,
    arrival_date,
    departure_date,
    feeding_type_id,
    cost,
    description
) VALUES (
    2,
    '2022-5-15',
    '2022-5-8',
    2,
    70005,
    'Хороший отель, уютные номера, все чистенько и опрятно. Завтраки хорошие, отличный фитнес центр и бассейн.'
);

INSERT INTO tour_order_payment_type (
    name
) VALUES (
    'Предоплата'
);

INSERT INTO tour_order_payment_type (
    name
) VALUES (
    'Кредит'
);

INSERT INTO tour_order_group DEFAULT VALUES;

INSERT INTO tour_order (
    client_id,
    payment_type_id,
    tour_id,
    price,
    people_count,
    group_id
) VALUES (
    1,
    1,
    1,
    70005,
    2,
    1
);
