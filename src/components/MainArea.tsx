import React from "react";

type MainAreaProps={
    memoContent : string;
    onContentChange:(content:string)=>void;
};


const MainArea=({memoContent,onContentChange}:MainAreaProps)=>{
    if(!memoContent){
        return <div className="main-area">メモを選択してください。</div>;
    }

    return(
        <div className="main-area">
            <textarea name="" id="" 
                    value={memoContent} 
                    onChange={(e)=>onContentChange(e.target.value)}>

            </textarea>
        </div>
    );
};

export default MainArea;