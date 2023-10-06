package blackforest;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.Random;

public class BlackForest {

//	public static void main(String[] args) {
//		Culture c1 = new Culture(VisitType.active, ForeignType.attack);
//		Culture c2 = new Culture(VisitType.active, ForeignType.friend);
//		for (int i = 0; i < 10000; i++) {
//			try {
//				c1.develop();
//			} catch (RuntimeException e) {
//				System.out.println(i);
//				throw e;
//			}
//		}
//		foreign(c1, c2);
//		System.out.println(c1.toString2() + ":" + c2.toString2());
//	}

        public static void main(String[] args) {
                // 随机建造10000个文明
                Random random = new Random();
                VisitType[] visitTypes = VisitType.values();
                ForeignType[] foreignTypes = ForeignType.values();

                List<Culture> activeCultures = new ArrayList<Culture>();
                List<Culture> allCultures = new ArrayList<Culture>();
                for (int i = 0; i < 10000; i++) {
                        VisitType visitType = visitTypes[random.nextInt(visitTypes.length)];
                        ForeignType foreignType = foreignTypes[random
                                        .nextInt(foreignTypes.length)];
                        Culture c = new Culture(visitType, foreignType);
                        allCultures.add(c);
                        if (visitType == VisitType.active)
                                activeCultures.add(c);
                }
                result(allCultures);

                // 经过500次的轮回
                for (int j = 0; j < 500; j++) {
                        // for (int j = 0; j < 10; j++) {
                        // 每个文明都能自己发展
                        for (Culture c : allCultures) {
                                c.develop();
                        }
                        // 激进的文明开始探索
                        for (Culture c1 : activeCultures) {
                                if (!c1.isExist())
                                        break;
                                Culture c2;
                                while (true) {
                                        c2 = allCultures.get(random.nextInt(allCultures.size()));
                                        if (c2 != c1 && c2.isExist())
                                                break;
                                }
                                foreign(c1, c2);
                        }
                        // 清除消失的文明
                        allCultures = delete(allCultures);
                        activeCultures = delete(activeCultures);
                        if (j % 50 == 0) {
                                System.out.print("第" + j + "轮：");
                                result(allCultures);
                        }
                        // 只有一个文明时，进化结束
                        if (allCultures.size() == 1) {
                                System.out.print("仅存一个文明，第" + j + "轮：");
                                result(allCultures);
                                break;
                        }
                        // 所有好奇的文明消失，进化结束
                        if (activeCultures.size() == 0) {
                                System.out.print("主动的文明全灭，第" + j + "轮：");
                                result(allCultures);
                                break;
                        }
                }
                result(allCultures);

        }

        private static List<Culture> delete(List<Culture> allCultures) {
                List<Culture> temp = new ArrayList<Culture>();
                for (Culture c : allCultures)
                        if (c.isExist())
                                temp.add(c);
                return temp;

        }

        // 文明间交流
        private static void foreign(Culture c1, Culture c2) {
                // long cc1 = c1.getCulture();
                // long cc2 = c2.getCulture();
                if (c1.foreignType == ForeignType.revenge) {
                        // 必须等别人进攻自己后才能报复
                        c2.foreignType.visit(c2, c1);
                        c1.foreignType.visit(c1, c2);
                } else {
                        c1.foreignType.visit(c1, c2);
                        c2.foreignType.visit(c2, c1);
                }
                // if (c1.getCulture() < 0 || c2.getCulture() < 0)
                // System.out.println("c1=" + cc1 + ":" + c1.toString2() + " c2="
                // + cc2 + ":" + c2.toString2());
        }

        // 統計結果
        private static void result(List<Culture> allCultures) {
                Map<Culture, Integer> result = new HashMap<Culture, Integer>();
                for (Culture c : allCultures) {
                        if (result.containsKey(c)) {
                                Integer a = result.get(c);
                                a++;
                                result.put(c, a);
                        } else {
                                result.put(c, 1);
                        }
                }
                System.out.println(result);

        }
}

// 探索策略：主动探索，被动等待
enum VisitType {
        active, passive
}

// 外交策略：进攻，友好，以牙还牙
enum ForeignType {
        attack {
                public void visit(Culture c1, Culture c2) {
                        if (c1.getCulture() > 0 && c2.getCulture() > 0) {
                                // 进攻这个文明
                                long s = (c1.getCulture() * Culture.a / 100);
                                if (s < 0)
                                        throw new IllegalStateException("s=" + s + " c1="
                                                        + c1.toString2());
                                c2.setCulture(c2.getCulture() - s);
                                // System.out.println(c1 + "把" + c2 + "消灭了"
                                // + (c1.culture * Culture.a / 100));
                        }
                }
        },
        friend {
                public void visit(Culture c1, Culture c2) {
                        if (c1.getCulture() > 0 && c2.getCulture() > 0) {
                                // 帮助这个文明
                                long s = (c1.getCulture() * Culture.h / 100);
                                if (s < 0)
                                        throw new IllegalStateException("s=" + s + " c1="
                                                        + c1.toString2());
                                // System.out.println("s=" + s + " c1=" + c1.toString2());
                                c2.setCulture(c2.getCulture() + s);
                                // System.out.println(c1 + "把" + c2 + "帮助了"
                                // + (c1.culture * Culture.h / 100));
                        }
                }
        },
        revenge {
                public void visit(Culture c1, Culture c2) {
                        if (c2.foreignType == attack)
                                attack.visit(c1, c2);
                        else
                                friend.visit(c1, c2);
                }
        };

        // c1对c2进行作用
        public abstract void visit(Culture c1, Culture c2);

}

class Culture {
        public final static int a = 1;
        public final static int h = 1;
        public final static int d = 0;

        private long culture = 1000000;
        VisitType visitType;
        ForeignType foreignType;// 

        Culture(VisitType visitType, ForeignType foreignType) {
                this.visitType = visitType;
                this.foreignType = foreignType;
        }

        public boolean isExist() {
                return culture > 0;
        }

        public long getCulture() {
                return culture;
        }

        public static long max = Long.MAX_VALUE / 10000;

        public void setCulture(long culture) {
                // System.out.println("culture="+culture);
                if (culture > max)
                        throw new IllegalStateException();

                this.culture = culture;
        }

        @Override
        public int hashCode() {
                return visitType.ordinal() * 10 + foreignType.ordinal();
        }

        @Override
        public boolean equals(Object o) {
                Culture c = (Culture) o;
                return c.visitType == visitType && c.foreignType == foreignType;
        }

        @Override
        public String toString() {
                // return visitType + ":" + foreignType + ":" + culture;
                return visitType + ":" + foreignType;
        }

        public String toString2() {
                return visitType + ":" + foreignType + ":" + culture;

        }

        public void develop() {
                setCulture(culture * d / 100 + culture);
        }
}
