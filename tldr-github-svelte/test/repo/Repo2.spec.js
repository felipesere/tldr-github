import {render, fireEvent} from "@testing-library/svelte";

import Repo from '../../src/repo/Repo.svelte'

describe("Second variation of the repo", () => {

    const sampleRepo = {
        activity: {
            prs: [{link: "foo", title: "a pr", by: "me"}],
            issues: [{link: "bar", title: "an issue", by: "you"}],
        }
    };

    it("shows three tabs", () => {
        const {getByText} = render(Repo, {repo: sampleRepo});

        expect(getByText("All")).toBeInTheDocument();
        expect(getByText("PRs")).toBeInTheDocument();
        expect(getByText("Issues")).toBeInTheDocument();

        expect(getByText("a pr")).toBeInTheDocument();
        expect(getByText("an issue")).toBeInTheDocument();
    });

    it("can pick the PR tab ", async () => {
        const {getByText, queryByText} = render(Repo, {repo: sampleRepo});

        const prs = getByText("PRs");

        await fireEvent.click(prs);
        expect(getByText("a pr")).toBeInTheDocument();
        expect(queryByText("an issue")).not.toBeInTheDocument();
    });

    it("can pick the issues tab ", async () => {
        const {getByText, queryByText} = render(Repo, {repo: sampleRepo});

        const issues = getByText("Issues");

        await fireEvent.click(issues);
        expect(getByText("an issue")).toBeInTheDocument();
        expect(queryByText("a pr")).not.toBeInTheDocument();
    });

    it("can go to the settings", async () => {
        const {getByTestId, getByText} = render(Repo, {repo: sampleRepo});

        const settings = getByTestId("settings");

        await fireEvent.click(settings);

        expect(getByText("Delete")).toBeInTheDocument();
    })
});
