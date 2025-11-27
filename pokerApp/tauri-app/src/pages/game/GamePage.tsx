import '@/styles/pages/auth/auth-pages.css';
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



export default function GamePage() {
     const userId = Cookie.get('userid');
     const [firstCard , setFirstCard]  = useState("");
     const [secondCard , setSecondCard]  = useState("");
     const [myMoney , setMyMoney]  = useState(0);
    //  const [bet , setBet]  = useState(0);
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
    async function handleBetting(e : any) {
        try {
            e.preventDefault();
             const resp = await axios.post(`${API_URL}/holdem/bet`, {
                player_id: userId,
                bet_size: document.getElementsByName("bet").values,
            });
        } catch (error) {
            
        }
    }
    async function handleGameStart() {
         try{
            const resp = await axios.post(`${API_URL}/holdem/start/solo`, {
                player_id: userId,
            });
            console.log('게임 시작 결과:', resp.data);
            const raw = resp.data.player_cards as string; 
            // 예: "[Card(Qc), Card(2d)]"

            const codes = raw.match(/[2-9TJQKA][scdh]/g) || [];
            console.log("raw:", raw);
            console.log("codes:", codes);

            const [firstCode, secondCode] = codes;
            console.log("firstCode:", firstCode, "secondCode:", secondCode);
            console.log(`테스트카드이미지 : ${card_images["2c"]}`)
            console.log(`카드이미지 :  ${card_images[firstCode || "값없음"]}`)
            console.log(`카드이미지2 :  ${card_images[secondCode || "값없음"]}`)
            
            setFirstCard(card_images[firstCode || "값없음"]);
            setSecondCard(card_images[secondCode || "값없음"]);
            console.log(`카드이미지2222  :  ${secondCard}`)

        } catch (error) {
            console.error('게임 시작 중 오류 발생:', error);
        } 
    }
    return (
        <>
            <button onClick={handleGameStart}>
                게임 시작하기
            </button>
        <div className="game-table-wrapper">
            {/* <img className="game-table-img" src={tableImg} alt="poker table" /> */}
            <div >
                <img src={firstCard} alt="" />
                <img src={secondCard} alt="" />

            </div>
            <div>
                <form action="" name='frm'>
                <input type='number' name='bet'>
                </input>
                <p>가진 돈 :  </p>
                <button onClick={handleBetting}>배팅하기</button>
                </form>
            </div>
        </div>
        <div>
            </div>
        </>
    );
}