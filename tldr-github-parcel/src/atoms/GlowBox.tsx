import * as React from "react";

type GlowBoxProps = {
    content: {
        link: string,
        title: string,
        by: string,
    },
}

export function GlowBox({content}: GlowBoxProps): JSX.Element {
    return (
        <div
            className="bg-gray-100 hover:bg-blue-100 flex-grow p-3 border border-gray-300 hover:border-blue-300 rounded-l">
            <a className="text-blue-600" href={content.link}>{content.title}</a>
            <p className="text-sm text-gray-500 font-light">by {content.by}</p>
        </div>
    )
}