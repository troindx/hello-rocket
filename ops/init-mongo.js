db.createUser({
    user: 'docker',
    pwd: 'mongopw',
    roles: [
        {
            role: 'readWrite',
            db: 'dispenser-api',
        },
    ],
});
    
db = new Mongo().getDB("dispenser_api");  
db.createCollection('dispensers', { capped: false });
db.createCollection('tabs', { capped: false });