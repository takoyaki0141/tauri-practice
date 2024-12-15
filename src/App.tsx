import React,{useState} from "react";
import Sidebar from "./components/Sidebar";
import MainArea from "./components/MainArea";

type Memo={
  id:number;
  title:string;
};

const App=()=>{
  const [memos,setMemos]=useState<Memo[]>([]);
  const [selectedMemoId,setSelectedMemoId]=useState<number|null>(null);
  const [memoContent,setMemoContent]=useState<string>("");


  const handleSelectMemo=(id:number)=>{
    setSelectedMemoId(id);
    //TODO:API空メモを取得する
  }

  const handleContentChange=(content:string)=>{
    setMemoContent(content);
    //TODO:自動保存処理を追加する
  }


  return(
    <div className="app">
      <Sidebar memos={memos} onSelectMemo={handleSelectMemo} />
      <MainArea memoContent={memoContent} onContentChange={handleContentChange}/>
    </div>
  );
};

export default App;