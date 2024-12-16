import React,{useState} from "react";
import Sidebar from "./components/Sidebar";
import MainArea from "./components/MainArea";

type Memo={
  id:number;
  title:string;
};

const App=()=>{

  //初期データとしてダミーのメモリストを設定
  const[memos,setMemos]=useState<Memo[]>([
    {id:1,title:"test1"},
    {id:2,title:"test2"},
    {id:3,title:"test3"}
  ])



  // const [memos,setMemos]=useState<Memo[]>([]);
  const [selectedMemoId,setSelectedMemoId]=useState<number|null>(null);
  const [memoContent,setMemoContent]=useState<string>("");


  const handleSelectMemo=(id:number)=>{
    setSelectedMemoId(id);
    //TODO:API空メモを取得する
    const selectedMemo=memos.find((memo)=>memo.id===id);
    setMemoContent(selectedMemo?selectedMemo.title:"no memo");
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