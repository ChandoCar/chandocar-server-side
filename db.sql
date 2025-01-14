CREATE TABLE USER (
                      ID INTEGER AUTO_INCREMENT PRIMARY KEY,
                      USERNAME VARCHAR(255) not null,
                      NAME VARCHAR(255) not null,
                      ADDRESS VARCHAR(255) not null,
                      ID_FAV_STOP INTEGER null
);

CREATE TABLE VEHICLE (
                         ID INTEGER AUTO_INCREMENT PRIMARY KEY,
                         PLATE VARCHAR(10) UNIQUE not null,-- Aquí va la matrícula la cuál deberá ser validada por el departemento de Programación
                         ID_USER INTEGER not null,
                         PLACE INTEGER not null,
                         BRAND VARCHAR(255) not null, -- Aquí va la marca del vehículo que tienen que verificar que exista
                         MODEL VARCHAR(255) not null, -- Aquí va el modelo del vehículo que tienen que verificar que exista
                         CONSTRAINT FK_VEHICLE_USER FOREIGN KEY (ID_USER) REFERENCES USER(ID)
);

CREATE TABLE LOCATION (
                          ID INTEGER AUTO_INCREMENT PRIMARY KEY,
                          LATITUDE DECIMAL(10,8) not null,
                          LONGITUDE DECIMAL(11,8) not null,
                          ID_USER INTEGER not null,
                          CONSTRAINT FK_LOCATION_USER FOREIGN KEY (ID_USER) REFERENCES USER(ID)
);

CREATE TABLE PASSENGER (
                           ID_USER INTEGER  not null ,
                           ID_VEHICLE INTEGER not null,
                           CONSTRAINT FK_PASSENGER_USER FOREIGN KEY (ID_USER) REFERENCES USER(ID) ON UPDATE CASCADE,
                           CONSTRAINT FK_PASSENGER_VEHICLE FOREIGN KEY (ID_VEHICLE) REFERENCES VEHICLE(ID) ON UPDATE CASCADE,
                           CONSTRAINT PK_PASSENGER PRIMARY KEY (ID_USER, ID_VEHICLE)
);

ALTER TABLE USER ADD CONSTRAINT FK_USER_FAV_STOP FOREIGN KEY (ID_FAV_STOP) REFERENCES LOCATION(ID)