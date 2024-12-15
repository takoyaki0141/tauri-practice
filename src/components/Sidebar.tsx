import React from "react";


type SidebarProps={
    memos:{id:number; title:string}[];
    onSelectMemo:(id:number)=>void;
}

const Sidebar=({memos,onSelectMemo}:SidebarProps)=>{
    return(
        <div className="sidebar">
            <ul>
                {memos.map((memo)=>(
                    <li key={memo.id} onClick={()=>onSelectMemo(memo.id)}>
                        {memo.title}
                    </li>
                ))}
            </ul>
        </div>
    )
}

export default Sidebar;