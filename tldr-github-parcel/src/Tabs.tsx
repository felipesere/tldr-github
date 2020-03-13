import * as React from "react";
import {GithubIcon, IconName} from "./atoms/GithubIcon";
import {useState} from "react";

type Tab = {
    value: TabNames,
    text: String,
    icon: IconName,
};

export type TabNames = "all" | "prs" | "issues";

const tabs: Tab[] = [
    {value: "all", text: "All", icon: false},
    {value: "prs", text: "PRs", icon: "git-pull-request"},
    {value: "issues", text: "Issues", icon: "issue-opened"}
];

export const defaultTab: TabNames = "all";

type TabsProps = {
    className: string,
    onChangeTab: (tab: TabNames) => void,
}

export function Tabs({className, onChangeTab}: TabsProps): JSX.Element {
    const [currentTab, setCurrentTab] = useState<TabNames>(defaultTab);
    const tabActivityClass = ({value}) =>
        value === currentTab ? "active" : "inactive";
    return (
        <ul className={`flex border-b list-none ${className}`}>
            {tabs.map(tab => {
                return (
                    <li className={tabActivityClass(tab)}>
                        <a
                            className="cursor-pointer"
                            onClick={() => {
                                setCurrentTab(tab.value);
                                onChangeTab(tab.value);
                            }}
                        >
                            <GithubIcon icon={tab.icon}/>
                            <span>{tab.text}</span>
                        </a>
                    </li>
                );
            })}
        </ul>
    );
}