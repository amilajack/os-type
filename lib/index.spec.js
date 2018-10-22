const addon = require('.');

describe('basic', () => {
    it('should return object', () => {
        expect(addon.currentPlatform()).toMatchSnapshot();
    });
});