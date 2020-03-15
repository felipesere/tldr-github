import * as React from "react";
import Octicons, {Gear, GitPullRequest, IssueOpened} from "@primer/octicons-react";

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
            return <Octicons icon={IssueOpened}/>;
        case "gear":
            return <Octicons icon={Gear}/>;
        case "git-pull-request":
            return <Octicons icon={GitPullRequest}/>;
        case false:
            return <div/>
    }
}