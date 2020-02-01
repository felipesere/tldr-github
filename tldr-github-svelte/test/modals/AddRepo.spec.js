import {render, fireEvent, cleanup} from "@testing-library/svelte";
import AddRepo from '../../src/modals/AddRepo.svelte'
import {enableMocks} from 'jest-fetch-mock'

enableMocks();

describe("add repo", () => {
    beforeEach(() => {
        fetchMock.resetMocks();
        cleanup()
    });

    it("bubbles up an event", async () => {
        fetchMock.mockReject("it failed");
        const {findByPlaceholderText, findByText, component} = render(AddRepo);
        component.$on('new-repo-added', (e) => console.log(e.detail));

        let input = await findByPlaceholderText("Add new repo");
        let button = await findByText("Add");

        await fireEvent.input(input, {target: {value: "foo/bar"}});
        await fireEvent.click(button);
    });
});