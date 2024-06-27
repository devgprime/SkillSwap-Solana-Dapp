exports.validateRequestBody = (req, res, next) => {
    if (!req.body) {
        return res.status(400).json({ error: 'Request body is empty' });
    }
    next();
};
