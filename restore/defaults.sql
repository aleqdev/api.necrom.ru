INSERT INTO worker_role (name) VALUES ('Управляющий отелем');
INSERT INTO worker_role (name) VALUES ('Администратор');

INSERT INTO worker (
    name, surname, last_name, phone_number, email, role_id
) VALUES (
    'Александр',
    'Александров',
    'Александрович',
    '73512662154',
    'alexander@gmail.com',
    1
);

INSERT INTO worker (
    name, surname, last_name, phone_number, email, role_id
) VALUES (
    'Максим',
    'Максимов',
    'Максимович',
    '78662983742',
    'maksim@gmail.com',
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

INSERT INTO db_user (
    email, password_hash
) VALUES (
    'primitive_email@not.even.valid',
    '$2b$12$agd/YFAsOd9Fsf.H82VnWuvhWGmf1uiUOAef4f1Vanpwo/x/6xe22'
);