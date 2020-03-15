import * as React from "react";
import Octicons, {gear, gitPullRequest, issueOpened} from "octicons-react";

export type IconName = "gear" | "git-pull-request" | "issue-opened" | false

type IconProps = {
    icon: IconName
}

export function GithubIcon({icon}: IconProps): JSX.Element {
    return <span className="svg-icon mr-1"><Icon icon={icon}/></span>
}

function Icon({icon}: IconProps): JSX.Element {
    switch (icon) {
        case "issue-opened":
            return <Octicons icon={issueOpened}/>;
        case "gear":
            return <Octicons icon={gear}/>;
        case "git-pull-request":
            return <Octicons icon={gitPullRequest}/>;
        case false:
            return <div/>
    }
}