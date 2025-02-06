-- Add migration script here


-- Insertar datos dummy en la tabla members_activities
INSERT INTO members_activities (member_id, activity_id) VALUES
((SELECT id FROM members WHERE ci = '12345678'), (SELECT id FROM activities WHERE name = 'Yoga')),
((SELECT id FROM members WHERE ci = '87654321'), (SELECT id FROM activities WHERE name = 'Fútbol')),
((SELECT id FROM members WHERE ci = '87654321'), (SELECT id FROM activities WHERE name = 'Yoga'));
