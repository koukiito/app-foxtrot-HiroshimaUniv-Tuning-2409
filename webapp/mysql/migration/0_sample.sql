-- このファイルに記述されたSQLコマンドが、マイグレーション時に実行されます。
ALTER TABLE users ADD INDEX index_users_on_username(username);
ALTER TABLE locations ADD INDEX index_locations_on_tow_truck_id(tow_truck_id);