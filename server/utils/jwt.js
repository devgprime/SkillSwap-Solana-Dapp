const jwt = require('jsonwebtoken');
const config = require('config');

exports.generateToken = (userId, username) => {
    return jwt.sign({ id: userId, username }, config.get('jwtSecret'));
};
