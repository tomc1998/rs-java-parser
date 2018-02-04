import java.util.List;
import java.util.Arrays;

public class Main {
  public static void main(String[] args) {
    /* This is a comment */
    System.out.println("Hello, world");

    Person p0 = new Person("John", 49);
    Person p1 = new Person("John", 49);
    Person p2 = new Person("John", 49);

    // This is another style of comment

    List<Person> personList = Arrays.asList(p0, p1, p2);

    for (Person p : personList) {
      System.out.println(p.getName() + ", aged " + p.getAge());
    }
  }
  class InnerClass {
    public int a;
  }
}

class PrivateClass {
  public int a;
}

enum PrivateEnum {
  A, B, C
}
