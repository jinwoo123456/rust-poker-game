import '@/styles/pages/game/game-page.css';
import tableImg from '@/assets/images/poker-table.png';
import { LoadingOverlay } from "@/utils/loading";
import { API_URL } from "@/utils/path";
import axios from "axios";
import Cookie from "js-cookie";
import { useCallback, useEffect, useState } from "react";
import { useLocation } from "react-router-dom";
// Card images - ranks: A,K,Q,J,T,9..2 ; suits: s(spades), h(hearts), d(diamonds), c(clubs)
import cardAs from "@/assets/images/Playing Cards/SVG-cards-1.3/ace_of_spades.svg";
import cardAc from "@/assets/images/Playing Cards/SVG-cards-1.3/ace_of_clubs.svg";
import cardAh from "@/assets/images/Playing Cards/SVG-cards-1.3/ace_of_hearts.svg";
import cardAd from "@/assets/images/Playing Cards/SVG-cards-1.3/ace_of_diamonds.svg";

import cardKs from "@/assets/images/Playing Cards/SVG-cards-1.3/king_of_spades.svg";
import cardKc from "@/assets/images/Playing Cards/SVG-cards-1.3/king_of_clubs.svg";
import cardKh from "@/assets/images/Playing Cards/SVG-cards-1.3/king_of_hearts.svg";
import cardKd from "@/assets/images/Playing Cards/SVG-cards-1.3/king_of_diamonds.svg";

import cardQs from "@/assets/images/Playing Cards/SVG-cards-1.3/queen_of_spades.svg";
import cardQc from "@/assets/images/Playing Cards/SVG-cards-1.3/queen_of_clubs.svg";
import cardQh from "@/assets/images/Playing Cards/SVG-cards-1.3/queen_of_hearts.svg";
import cardQd from "@/assets/images/Playing Cards/SVG-cards-1.3/queen_of_diamonds.svg";

import cardJs from "@/assets/images/Playing Cards/SVG-cards-1.3/jack_of_spades.svg";
import cardJc from "@/assets/images/Playing Cards/SVG-cards-1.3/jack_of_clubs.svg";
import cardJh from "@/assets/images/Playing Cards/SVG-cards-1.3/jack_of_hearts.svg";
import cardJd from "@/assets/images/Playing Cards/SVG-cards-1.3/jack_of_diamonds.svg";

import cardTs from "@/assets/images/Playing Cards/SVG-cards-1.3/10_of_spades.svg";
import cardTc from "@/assets/images/Playing Cards/SVG-cards-1.3/10_of_clubs.svg";
import cardTh from "@/assets/images/Playing Cards/SVG-cards-1.3/10_of_hearts.svg";
import cardTd from "@/assets/images/Playing Cards/SVG-cards-1.3/10_of_diamonds.svg";

import card9s from "@/assets/images/Playing Cards/SVG-cards-1.3/9_of_spades.svg";
import card9c from "@/assets/images/Playing Cards/SVG-cards-1.3/9_of_clubs.svg";
import card9h from "@/assets/images/Playing Cards/SVG-cards-1.3/9_of_hearts.svg";
import card9d from "@/assets/images/Playing Cards/SVG-cards-1.3/9_of_diamonds.svg";

import card8s from "@/assets/images/Playing Cards/SVG-cards-1.3/8_of_spades.svg";
import card8c from "@/assets/images/Playing Cards/SVG-cards-1.3/8_of_clubs.svg";
import card8h from "@/assets/images/Playing Cards/SVG-cards-1.3/8_of_hearts.svg";
import card8d from "@/assets/images/Playing Cards/SVG-cards-1.3/8_of_diamonds.svg";

import card7s from "@/assets/images/Playing Cards/SVG-cards-1.3/7_of_spades.svg";
import card7c from "@/assets/images/Playing Cards/SVG-cards-1.3/7_of_clubs.svg";
import card7h from "@/assets/images/Playing Cards/SVG-cards-1.3/7_of_hearts.svg";
import card7d from "@/assets/images/Playing Cards/SVG-cards-1.3/7_of_diamonds.svg";

import card6s from "@/assets/images/Playing Cards/SVG-cards-1.3/6_of_spades.svg";
import card6c from "@/assets/images/Playing Cards/SVG-cards-1.3/6_of_clubs.svg";
import card6h from "@/assets/images/Playing Cards/SVG-cards-1.3/6_of_hearts.svg";
import card6d from "@/assets/images/Playing Cards/SVG-cards-1.3/6_of_diamonds.svg";

import card5s from "@/assets/images/Playing Cards/SVG-cards-1.3/5_of_spades.svg";
import card5c from "@/assets/images/Playing Cards/SVG-cards-1.3/5_of_clubs.svg";
import card5h from "@/assets/images/Playing Cards/SVG-cards-1.3/5_of_hearts.svg";
import card5d from "@/assets/images/Playing Cards/SVG-cards-1.3/5_of_diamonds.svg";

import card4s from "@/assets/images/Playing Cards/SVG-cards-1.3/4_of_spades.svg";
import card4c from "@/assets/images/Playing Cards/SVG-cards-1.3/4_of_clubs.svg";
import card4h from "@/assets/images/Playing Cards/SVG-cards-1.3/4_of_hearts.svg";
import card4d from "@/assets/images/Playing Cards/SVG-cards-1.3/4_of_diamonds.svg";

import card3s from "@/assets/images/Playing Cards/SVG-cards-1.3/3_of_spades.svg";
import card3c from "@/assets/images/Playing Cards/SVG-cards-1.3/3_of_clubs.svg";
import card3h from "@/assets/images/Playing Cards/SVG-cards-1.3/3_of_hearts.svg";
import card3d from "@/assets/images/Playing Cards/SVG-cards-1.3/3_of_diamonds.svg";

import card2s from "@/assets/images/Playing Cards/SVG-cards-1.3/2_of_spades.svg";
import card2c from "@/assets/images/Playing Cards/SVG-cards-1.3/2_of_clubs.svg";
import card2h from "@/assets/images/Playing Cards/SVG-cards-1.3/2_of_hearts.svg";
import card2d from "@/assets/images/Playing Cards/SVG-cards-1.3/2_of_diamonds.svg";
import chips from "@/assets/images/chips.png";




export default function GamePage() {
    const userId = Cookie.get('userid');
    const [firstCard, setFirstCard] = useState("");
    const [secondCard, setSecondCard] = useState("");
    const [botFirstCard, setBotFirstCard] = useState("");
    const [botSecondCard, setBotSecondCard] = useState("");
    const [gameId, setGameId] = useState("");
    const [myMoney, setMyMoney] = useState(20000);
    const [botMoney, setBotMoney] = useState(20000);
    const [communitiyCard1, setCommunitiyCard1] = useState("");
    const [communitiyCard2, setCommunitiyCard2] = useState("");
    const [communitiyCard3, setCommunitiyCard3] = useState("");
    const [communitiyCard4, setCommunitiyCard4] = useState("");
    const [communitiyCard5, setCommunitiyCard5] = useState("");
    const [gameState, setGameState] = useState("blind");
    const [bet, setBet] = useState<number>(0);
    const [botAction, setBotAction] = useState("");
    const [botBet, setBotBet] = useState(0);
    const [pot, setPot] = useState(0);
    const [playerAction, setPlayerAction] = useState("");
    const [playerBet, setPlayerBet] = useState(0);
    const [isCardOpen, setIsCardOpen] = useState(false); // 콜이 나올 경우 카드 오픈( 콜일 경우 True)
    const [gameCount, setGameCount] = useState<number>(0)
    // 승자 계산하는 용으로 카드 식별 저장 (As, 5d 이런것들)
    const [firstCardCode, setFirstCardCode] = useState("");
    const [secondCardCode, setSecondCardCode] = useState("");
    const [botFirstCardCode, setBotFirstCardCode] = useState("");
    const [botSecondCardCode, setBotSecondCardCode] = useState("");

    // 게임 진행 상태
    const [isGameProgress, setIsGameProgress] = useState<boolean>(false);
    const [communityCardCodes, setCommunityCardCodes] = useState<string[]>([]);


    // 칩 애니메이션 표시 여부
    const [showPlayerChipAnim, setShowPlayerChipAnim] = useState(false);
    const [showBotChipAnim, setShowBotChipAnim] = useState(false);

    // 가운데 보여줄 텍스트
    const [centerMessage, setCenterMessage] = useState("");
    const [messageQueue, setMessageQueue] = useState<string[]>([]);


    const card_images: Record<string, string> = {
        // Aces
        As: cardAs,
        Ac: cardAc,
        Ah: cardAh,
        Ad: cardAd,
        // Kings
        Ks: cardKs,
        Kc: cardKc,
        Kh: cardKh,
        Kd: cardKd,
        // Queens
        Qs: cardQs,
        Qc: cardQc,
        Qh: cardQh,
        Qd: cardQd,
        // Jacks
        Js: cardJs,
        Jc: cardJc,
        Jh: cardJh,
        Jd: cardJd,
        // Tens (T)
        Ts: cardTs,
        Tc: cardTc,
        Th: cardTh,
        Td: cardTd,
        // Nines
        '9s': card9s,
        '9c': card9c,
        '9h': card9h,
        '9d': card9d,
        // Eights
        '8s': card8s,
        '8c': card8c,
        '8h': card8h,
        '8d': card8d,
        // Sevens
        '7s': card7s,
        '7c': card7c,
        '7h': card7h,
        '7d': card7d,
        // Sixes
        '6s': card6s,
        '6c': card6c,
        '6h': card6h,
        '6d': card6d,
        // Fives
        '5s': card5s,
        '5c': card5c,
        '5h': card5h,
        '5d': card5d,
        // Fours
        '4s': card4s,
        '4c': card4c,
        '4h': card4h,
        '4d': card4d,
        // Threes
        '3s': card3s,
        '3c': card3c,
        '3h': card3h,
        '3d': card3d,
        // Twos
        '2s': card2s,
        '2c': card2c,
        '2h': card2h,
        '2d': card2d,
    };

    // 가운데 텍스트 보여줫다 사라지게 하는 함수 
    function showCenterAction(message: string) {
        setCenterMessage(message);
        setTimeout(() => {
            setCenterMessage("");
        }, 4000); //
    }
    function botRandomAction(playerBet: number) {
        if (botMoney <= 0) {
            setBotAction("올인 후 잔액 0");
            showCenterAction("봇: 올인");
            return;
        }

        const r = Math.random(); // 0 이상 1 미만

        // 0 ~ 1 사이 랜덤 값으로 행동 결정
        // 0.0 ~ 0.3 : 폴드
        // 0.3 ~ 0.8 : 콜
        // 0.8 ~ 1.0 : 레이즈
        if (r < 0.1) {
            setBotAction("폴드");
            setBotBet(0);
            setMyMoney(prev => prev + pot);
            setPot(0);
            console.log("봇: 폴드");
            showCenterAction("봇: 폴드");
            gameRoundEnd();
        } else if (r < 0.8) {
            let callAmount = playerBet;
            if (callAmount > botMoney) {
                callAmount = botMoney;
            }

            setBotMoney(prev => prev - callAmount);
            setBotBet(prev => prev + callAmount);
            setPot(prev => prev + callAmount);

            setBotAction("콜");

            console.log(`봇: 콜 ${callAmount}`);
            showCenterAction(`봇: 콜 ${callAmount}`);
            setShowBotChipAnim(false);
            setTimeout(() => setShowBotChipAnim(true), 0);

            openCard();
        } else {
            let raiseAmount = playerBet + Math.floor(Math.random() * 50) + 10;
            if (raiseAmount > botMoney) {
                raiseAmount = botMoney;
            }
            setBotMoney(prev => prev - raiseAmount);
            setBotBet(raiseAmount);
            setBotAction(`${raiseAmount}원 레이즈`);
            setPot(prev => prev + raiseAmount);
            console.log(`봇: 레이즈 ${raiseAmount}`);
            showCenterAction(`봇: 레이즈 ${raiseAmount}`);
            setShowBotChipAnim(false);
            setTimeout(() => setShowBotChipAnim(true), 0);

        }
    }
    function botActionAfterCheck() {
        if (botMoney <= 0) {
            setBotAction("체크");
            showCenterAction("봇: 체크");
            console.log("봇: 체크 (잔액 0)");
            openCard();
            return;
        }

        const r = Math.random();

        if (r < 0.5) {
            setBotAction("체크");
            showCenterAction("봇: 체크");
            console.log("봇: 체크");

            openCard();
        } else {
            let betAmount = Math.floor(Math.random() * 50) + 10; // 랜덤 베팅
            if (betAmount > botMoney) {
                betAmount = botMoney;
            }

            setBotMoney(prev => prev - betAmount);
            setBotBet(betAmount);
            setPot(prev => prev + betAmount);

            setBotAction(`${betAmount}원 배팅`);
            showCenterAction(`봇: 배팅 ${betAmount}`);
            console.log(`봇: 배팅 ${betAmount}`);

            setShowBotChipAnim(false);
            setTimeout(() => setShowBotChipAnim(true), 0);

        }
    }


    async function handleBetting(e: React.FormEvent<HTMLFormElement>) {

        e.preventDefault();
        // if (communitiyCard1 != "" && communitiyCard4 == "") {
        //     setGameState("flop");
        // }
        // else if (communitiyCard1 != "" && communitiyCard4 != "" && communitiyCard5 == "") {
        //     setGameState("turn");
        // }
        // else if (communitiyCard1 != "" && communitiyCard4 != "" && communitiyCard5 != "") {
        //     setGameState("river");
        // }
        if (bet > myMoney) {
            alert("가진 금액보다 큰 배팅은 할 수 없습니다.");
            setBet(0);
            return false;
        }
        setShowPlayerChipAnim(false);
        setTimeout(() => setShowPlayerChipAnim(true), 0);
        setMyMoney(prev => prev - bet);
        setPlayerBet(bet);
        setPlayerAction(` ${bet}원 배팅`);
        setPot(prev => prev + bet);

        await new Promise(resolve => setTimeout(resolve, 2000));
        botRandomAction(bet);
        // 프론트에서 배팅처리 전부 처리하게 변경!! 

        // const resp = await axios.post(`${API_URL}/holdem/player/action`, {
        //     playerId: userId,
        //     active: "1", // 배팅 후 상대 배팅 가능 여부 
        //     betSize: bet,
        //     gameStats: gameState,
        //     gameId: gameId,

        // });

    }

    function handlePlayerFold() {
        // 플레이어가 포기 → 봇이 팟 가져감
        if (pot > 0) {
            setBotMoney(prev => prev + pot);
            setPot(0);
        }
        setPlayerAction("폴드");
        setPlayerBet(0);
        console.log("플레이어: 폴드");
        showCenterAction("플레이어: 폴드");
        gameRoundEnd();
    }

    function handlePlayerCall() {
        if (botBet <= 0) {
            setPlayerAction("체크");
            setPlayerBet(0);
            console.log("플레이어: 체크");
            showCenterAction("플레이어: 체크");
            openCard();
            setTimeout(() => {
                botActionAfterCheck();
            }, 1500);

            return;
        }

        setPlayerAction("콜");

        let callAmount = botBet;
        if (callAmount > myMoney) {
            callAmount = myMoney;
        }

        setMyMoney(prev => prev - callAmount);
        setPlayerBet(callAmount);
        setPot(prev => prev + callAmount);

        setShowPlayerChipAnim(false);
        setTimeout(() => setShowPlayerChipAnim(true), 0);

        console.log(`플레이어: 콜 ${callAmount}`);
        showCenterAction(`플레이어: 콜 ${callAmount}`);



        openCard();
    }

    function gameRoundEnd() {
        let card1 = document.getElementById("card1");
        let card2 = document.getElementById("card2");
        let card3 = document.getElementById("card3");
        let card4 = document.getElementById("card4");
        let card5 = document.getElementById("card5");
        if (card1) card1.classList.add("hide");
        if (card2) card2.classList.add("hide");
        if (card3) card3.classList.add("hide");
        if (card4) card4.classList.add("hide");
        if (card5) card5.classList.add("hide");

        setBotFirstCard("");
        setBotSecondCard("");
        setFirstCard("");
        setSecondCard("");
        setCommunitiyCard1("");
        setCommunitiyCard2("");
        setCommunitiyCard3("");
        setCommunitiyCard4("");
        setCommunitiyCard5("");
        setPot(0);

        setGameState("blind");
        setPlayerBet(0);
        setBotBet(0);
        setPlayerAction("");
        setBotAction("");
        setIsGameProgress(false);
    }


    function openCard() {
        setGameState((prev) => {
            if (prev === "blind") return "flop";
            if (prev === "flop") return "turn";
            if (prev === "turn") return "river";
            if (prev === "river") return "showdown";
            return prev;
        });
        console.log(`보드 상태 : ${gameState}`)
        // 다음 스트리트로 넘어갈 때, 이 스트리트의 베팅은 초기화
        setPlayerBet(0);
        setBotBet(0);
    }

    async function getWhoWin() {
        console.log(`커뮤니티카드 프론트 : ${communityCardCodes}`)

        try {
            const resp = await axios.post(`${API_URL}/holdem/winner`, {
                playerCards: [firstCardCode, secondCardCode],
                botCards: [botFirstCardCode, botSecondCardCode],
                communityCards: communityCardCodes,
            });
            console.log("쇼다운결과 = ", resp);
            let winner = resp.data.result;
            alert(`${winner} 가 승리하였습니다! 팟이 모두 ${winner} 에게 갑니다.`);

            if (winner == "player") {

                setMyMoney(prev => prev + pot);

            }
            else {
                setBotMoney(prev => prev + pot);

            }
            setPot(0);
            setGameState("blind");
            setIsGameProgress(false);
        } catch (error) {

        }
    }
    async function handleGameStart() {
        setGameState("blind");
        setPlayerBet(0);
        setBotBet(0);
        setPot(0);
        setPlayerAction("");
        setBotAction("");
        try {
            const resp = await axios.post(`${API_URL}/holdem/start/solo`, {
                player_id: userId,
            });
            console.log('게임 시작 결과:', resp.data);
            const raw = resp.data.player_cards as string;
            const rawCommunityCard = resp.data.comunity_cards as string;
            const rawBotCard = resp.data.bot_cards as string;
            // 예: "[Card(Qc), Card(2d)]"

            const codes = raw.match(/[2-9TJQKA][scdh]/g) || [];
            const codes2 = rawCommunityCard.match(/[2-9TJQKA][scdh]/g) || [];
            const codes3 = rawBotCard.match(/[2-9TJQKA][scdh]/g) || [];

            console.log("raw:", raw);
            console.log("raw:", rawCommunityCard);
            console.log("codes:", codes);
            console.log("codes2:", codes2);
            console.log("codes3:", codes3);
            console.log("gamecount", gameCount);
            if (gameCount < 1) {

                setBotMoney(resp.data.bot_money);
                setMyMoney(resp.data.player_money);
            }

            console.log(`봇돈 =  ${botMoney}`);
            console.log(`내 돈 =  ${myMoney}`);

            const [firstCode, secondCode] = codes;
            const [cCard1, cCard2, cCard3, cCard4, cCard5] = codes2;
            const [firstBotCard, secondBotCard] = codes3;

            console.log(`공유패 =  ${cCard1}`);
            // 승자 계산용 코드 저장
            setFirstCardCode(firstCode || "");
            setSecondCardCode(secondCode || "");
            setBotFirstCardCode(firstBotCard || "");
            setBotSecondCardCode(secondBotCard || "");
            setCommunityCardCodes(codes2 || "");

            setCommunitiyCard1(card_images[cCard1 || "값없음"]);
            setCommunitiyCard2(card_images[cCard2 || "값없음"]);
            setCommunitiyCard3(card_images[cCard3 || "값없음"]);
            setCommunitiyCard4(card_images[cCard4 || "값없음"]);
            setCommunitiyCard5(card_images[cCard5 || "값없음"]);

            setBotFirstCard(card_images[firstBotCard || "값없음"]);
            setBotSecondCard(card_images[secondBotCard || "값없음"]);
            setPot(resp.data.pot);

            console.log(" 게임 팟 : ", pot);
            console.log("cCard1  = ", cCard1);
            console.log("firstCode:", firstCode, "secondCode:", secondCode);
            console.log(`테스트카드이미지 : ${card_images["2c"]}`)
            console.log(`카드이미지 :  ${card_images[firstCode || "값없음"]}`)
            console.log(`카드이미지2 :  ${card_images[secondCode || "값없음"]}`)

            setFirstCard(card_images[firstCode || "값없음"]);
            setSecondCard(card_images[secondCode || "값없음"]);
            console.log(` 봇배팅 ${resp.data.bot_money}`)
            console.log(` 플레이어 배팅 ${resp.data.player_Money}`)
            showCenterAction(`게임을 시작합니다!`);
            // setBotMoney(resp.data.bot_money);
            // setMyMoney(resp.data.player_money);
            setGameId(resp.data.game_id);
            console.log(`카드이미지2222  :  ${secondCard}`)
            setGameCount(prev => prev + 1);
            setIsGameProgress(true);
        } catch (error) {
            console.error('게임 시작 중 오류 발생:', error);
        }
    }

    useEffect(() => {
        if (gameState === "showdown") {
            getWhoWin();
        }
    }, [gameState]);
    return (
        <>
            <div className='background-table-img'>
                {/* <img className='table-img' src={tableImg} alt="" /> */}
                {/* <button onClick={getWhoWin}>
                결과 확인하기
                </button> */}
                <div className='game-container'>
                    <button className='start-game-btn' onClick={handleGameStart} >
                        게임 시작하기
                    </button>
                    <div> game id : {gameId} </div>
                    <div className='player-section'>

                        {/* <p> 봇 카드 : </p> */}
                        <div className='hands'>
                            <img src={botFirstCard} className='game_table_card_size' alt="" />
                            <img src={botSecondCard} className='game_table_card_size' alt="" />
                        </div>
                        <div className='action'>
                            {/* <p>가진 돈 :  {botMoney}</p>
                                    <p>액션 : {botAction}</p> */}
                        </div>

                    </div>
                    <div className='game-table-section'>
                        <div className="game-table-wrapper">
                            <p>팟 : {pot}</p>
                            {centerMessage && (
                                <div className="center-action-message">
                                    {centerMessage}
                                </div>
                            )}
                            {showBotChipAnim && (
                                <img
                                    src={chips}
                                    alt="bot bet chips"
                                    className="chip-anim chip-anim-bot"
                                    onAnimationEnd={() => setShowBotChipAnim(false)}
                                />
                            )}

                            {/* 🔽 플레이어 칩 (아래에서 위로) */}
                            {showPlayerChipAnim && (
                                <img
                                    src={chips}
                                    alt="player bet chips"
                                    className="chip-anim chip-anim-player"
                                    onAnimationEnd={() => setShowPlayerChipAnim(false)}
                                />
                            )}
                            {/* <img className="game-table-img" src={tableImg} alt="poker table" /> */}
                            <div style={{ display: 'none' }}>
                            </div>
                            {gameState !== "blind" && (
                                <>
                                    <img src={communitiyCard1} id='card1' alt="" className='game_table_card_size' />
                                    <img src={communitiyCard2} id='card2' alt="" className='game_table_card_size' />
                                    <img src={communitiyCard3} id='card3' alt="" className='game_table_card_size' />
                                </>
                            )}

                            {(gameState === "turn" || gameState === "river") && (
                                <img src={communitiyCard4} id='card4' alt="" className='game_table_card_size' />
                            )}

                            {gameState === "river" && (
                                <img src={communitiyCard5} id='card5' alt="" className='game_table_card_size' />
                            )}
                        </div>
                    </div>
                    <div className='player-section'>
                        <div className='hands'>
                            <img src={firstCard} className='game_table_card_size' alt="" />
                            <img src={secondCard} className='game_table_card_size' alt="" />
                        </div>
                        <div className='action'>
                            <form action="" name='frm' onSubmit={handleBetting}>
                                {/* <p> 플레이어 액션 :  {playerAction}</p> */}

                                <input type='hidden' name='gameId' value={gameId} />
                                {isGameProgress == true && (
                                    <>
                                        <div className='btn-box'>
                                            <button className='bet-btn' type="button" name='fold' onClick={handlePlayerFold}>폴드</button>
                                            <button className='bet-btn' type="button" name='call' onClick={handlePlayerCall}>체크/콜</button>
                                            <button className='bet-btn' type='submit' >배팅하기</button>
                                        </div>
                                        {/* <button name='raise'>레이즈</button> */}
                                        <div className="bet-range-wrapper">
                                            <input
                                                type="range"
                                                name="bet"
                                                value={bet}
                                                min={200}
                                                max={myMoney}
                                                onChange={(e) => {
                                                    const value = Number(e.target.value);
                                                    setBet(isNaN(value) ? 0 : value);
                                                }}
                                            />
                                            <span className="bet-value">{bet} 원</span>
                                        </div>


                                    </>
                                )}

                                {/* <p>가진 돈 :  {myMoney}</p> */}
                            </form>
                        </div>
                    </div>

                </div>
            </div>
        </>
    );
}