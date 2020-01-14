import "@testing-library/jest-dom/extend-expect";
import { render, fireEvent } from "@testing-library/svelte";

import LastCommit from "LastCommit";

describe("the last commit", () => {
  it("shows the comment", () => {
    let props = {
      lastCommit: {
        branch: "foo",
        on: "12/2/2020",
        by: "Felpe",
        comment: "Something was done"
      }
    };
    const { getByText } = render(LastCommit, props);

    expect(getByText("Something was done")).toBeInTheDocument();
  });
});
