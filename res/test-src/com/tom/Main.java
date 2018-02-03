import java.util.List;
import java.util.Arrays;

public class Main {
  public static void main(String[] args) {
    System.out.println("Hello, world");

    Person p0 = new Person("John", 49);
    Person p1 = new Person("John", 49);
    Person p2 = new Person("John", 49);

    List<Person> personList = Arrays.asList(p0, p1, p2);

    for (Person p : personList) {
      System.out.println(p.getName() + ", aged " + p.getAge());
    }
  }
}
