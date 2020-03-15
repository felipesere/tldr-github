import * as React from "react";
import {Tabs} from './Tabs'
import {render, fireEvent, screen} from '@testing-library/react'

const findActive = (container: HTMLElement): String => {
    const el = container.querySelector('.active');

    return el ? el.textContent || "" : "not found";
};

describe("the tabs", () => {
    it("defaults to 'all'", async () => {
        const {container} = render(<Tabs onChangeTab={() => {
        }}/>);

        expect(findActive(container)).toEqual("All");
    });

    it("switch when clicked", async () => {
        const changeSpy = jest.fn();
        const {container} = render(<Tabs onChangeTab={changeSpy}/>);

        fireEvent.click(await screen.findByText('PRs'));

        expect(findActive(container)).toEqual("PRs");
        expect(changeSpy).toHaveBeenCalledWith("prs");
    });
});
