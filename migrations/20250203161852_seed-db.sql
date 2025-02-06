-- Insertar datos dummy en la tabla space
INSERT INTO space (name) VALUES
('Cancha Interior'),
('Cancha Exterior'),
('Parrillero');

-- Insertar datos dummy en la tabla medical_society
INSERT INTO medical_society (name, emergency_phone) VALUES
('SEMM', '159');

-- Insertar datos dummy en la tabla members
INSERT INTO members (name, lastname, ci, birth_date, phone, tutor_name, tutor_lastname, tutor_phone, observation, medical_society_id, address) VALUES
('Juan', 'Pérez', '12345678', '1990-01-01', '555-1234', 'Carlos', 'Pérez', '555-5678', 'Ninguna', (SELECT id FROM medical_society WHERE name = 'SEMM'), 'Calle Falsa 123'),
('Ana', 'Gómez', '87654321', '1985-05-15', '555-4321', NULL, NULL, NULL, 'Alergia a los frutos secos', (SELECT id FROM medical_society WHERE name = 'SEMM'), 'Avenida Siempre Viva 742');

-- Insertar datos dummy en la tabla dues
INSERT INTO dues (member_id, amount, payment_date, month, year, is_payed) VALUES
((SELECT id FROM members WHERE ci = '12345678'), 100.00, '2023-01-10', 1, 2023, TRUE),
((SELECT id FROM members WHERE ci = '87654321'), 100.00, '2023-01-15', 1, 2023, FALSE);

-- Insertar datos dummy en la tabla activities
INSERT INTO activities (name, category) VALUES
('Yoga', 'Bienestar'),
('Pilates', 'Bienestar'),
('Fútbol', 'Deportes');

-- Insertar datos dummy en la tabla members_activities
INSERT INTO members_activities (member_id, activity_id) VALUES
((SELECT id FROM members WHERE ci = '12345678'), (SELECT id FROM activities WHERE name = 'Yoga')),
((SELECT id FROM members WHERE ci = '87654321'), (SELECT id FROM activities WHERE name = 'Fútbol'));

-- Insertar datos dummy en la tabla activities_schedule
INSERT INTO activities_schedule (activity_id, day, start_time, end_time, space_id) VALUES
((SELECT id FROM activities WHERE name = 'Yoga'), 'Lunes', '08:00', '09:00', (SELECT id FROM space WHERE name = 'Cancha Interior')),
((SELECT id FROM activities WHERE name = 'Fútbol'), 'Miércoles', '17:00', '19:00', (SELECT id FROM space WHERE name = 'Cancha Exterior'));

-- Insertar datos dummy en la tabla rents
INSERT INTO rents (full_name, phone, start_time, end_time, space_id, cost, payment_date, is_payed) VALUES
('Carlos Ruiz', '555-9876', '14:00', '16:00', (SELECT id FROM space WHERE name = 'Parrillero'), 200.00, '2023-01-20', TRUE),
('Marta Sánchez', '555-6543', '10:00', '12:00', (SELECT id FROM space WHERE name = 'Cancha Interior'), 150.00, '2023-01-22', FALSE);

-- Insertar datos dummy en la tabla users
INSERT INTO users (name, rolename, email, password) VALUES
('admin', 'Administrador', 'admin@example.com', 'securepassword'),
('user', 'Usuario', 'user@example.com', 'userpassword');

-- Insertar datos dummy en la tabla employees
INSERT INTO employees (name, lastname, ci, phone, address, medical_society_id) VALUES
('Luis', 'Martínez', '11223344', '555-1111', 'Calle Luna 45', (SELECT id FROM medical_society WHERE name = 'SEMM')),
('Sofía', 'Hernández', '44332211', '555-2222', 'Calle Sol 12', (SELECT id FROM medical_society WHERE name = 'SEMM'));

-- Insertar datos dummy en la tabla employees_payments
INSERT INTO employees_payments (employee_id, amount, payment_date, month, year, is_payed) VALUES
((SELECT id FROM employees WHERE ci = '11223344'), 1200.00, '2023-01-30', 1, 2023, TRUE),
((SELECT id FROM employees WHERE ci = '44332211'), 1200.00, '2023-01-30', 1, 2023, FALSE);