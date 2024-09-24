-- このファイルに記述されたSQLコマンドが、マイグレーション時に実行されます。
ALTER TABLE users ADD INDEX index_users_on_username(username);
ALTER TABLE locations ADD INDEX index_locations_on_tow_truck_id(tow_truck_id);
ALTER TABLE edges ADD node_a_area_id INT;
UPDATE edges SET node_a_area_id = (SELECT area_id FROM nodes WHERE nodes.id = edges.node_a_id);
ALTER TABLE edges ADD INDEX index_edges_on_node_a_area_id(node_a_area_id);
ALTER TABLE dispatchers ADD INDEX index_dispatchers_on_user_id(user_id);