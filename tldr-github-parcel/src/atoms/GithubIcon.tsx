import * as React from "react";

export type IconName = "gear" | "git-pull-request" | "issue-opened" | false

type IconProps = {
    icon: IconName
}

export function GithubIcon({icon: IconName}: IconProps): JSX.Element {
    return <div />
}