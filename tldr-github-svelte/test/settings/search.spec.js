import {search} from '../../src/settings/search.js'

describe('the search', () => {
    it('matches on simple fields', () => {
        const items = [{word: "foo"}, {word: "bar"}];
        const term = new RegExp("fo", "i");

        const results = search(items, term, ["word"]);

        expect(results).toContainEqual({word: "foo"})
    });

    it('matches on array fields', () => {
        const items = [{word: ["foo", "apple"]}, {word: ["bar", "banana"]}];
        const term = new RegExp("fo", "i");

        const results = search(items, term, ["word"]);

        expect(results).toContainEqual({word: ["foo", "apple"]})
    });

    it('ignores values that dont have the field', () => {
        const items = [{}, {notWord: 1}, {word: "foo"}, {word: "bar"}];
        const term = new RegExp("fo", "i");

        const results = search(items, term, ["word"]);

        expect(results).toContainEqual({word: "foo"})
    });
});