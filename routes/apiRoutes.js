const express = require('express');
const router = express.Router();
const userController = require('../controllers/userController');
const { authenticateUser } = require('../middlewares/authMiddleware');

router.get('/users/:userId', authenticateUser, userController.getUserById);

module.exports = router;

