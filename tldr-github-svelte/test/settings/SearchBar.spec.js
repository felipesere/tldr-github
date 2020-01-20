import { bind } from "svelte/internal";
import {render, fireEvent, cleanup} from "@testing-library/svelte";
import SearchBar from '../../src/settings/SearchBar.svelte'

describe("the search bar", () => {
    beforeEach(cleanup);

    it('narrows down items', async () => {
        const items = [
            {link: "foo", title: "a pr", by: "me", labels: ["apple", "banana"]},
            {link: "bar", title: "an issue", by: "me", labels: ["lemon"]},
        ];

        const {findByPlaceholderText, component} = render(SearchBar, {items});

        let searchResults = [];
        bind(component, "searchResults", (newResults) => searchResults = newResults);

        let term = await findByPlaceholderText('Search...');
        await fireEvent.input(term, {target: {value: "issue"}});

        expect(searchResults).toContainEqual(items[1])
    })
});
