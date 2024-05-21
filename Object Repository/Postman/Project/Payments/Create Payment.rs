<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Create Payment</name>
   <tag></tag>
   <elementGuidId>60b9db1c-417d-48e5-8cd7-5f03fddad63d</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;key\&quot; : \&quot;123456\&quot;,\n  \&quot;interfaceId\&quot; : \&quot;789011\&quot;,\n  \&quot;amountPlanned\&quot; : {\n    \&quot;currencyCode\&quot; : \&quot;USD\&quot;,\n    \&quot;centAmount\&quot; : 1000\n  },\n  \&quot;paymentMethodInfo\&quot; : {\n    \&quot;paymentInterface\&quot; : \&quot;STRIPE\&quot;,\n    \&quot;method\&quot; : \&quot;CREDIT_CARD\&quot;,\n    \&quot;name\&quot; : {\n      \&quot;en\&quot; : \&quot;Credit Card\&quot;\n    }\n  },\n  \&quot;transactions\&quot; : [ {\n    \&quot;timestamp\&quot; : \&quot;2015-10-20T08:54:24.000Z\&quot;,\n    \&quot;type\&quot; : \&quot;Charge\&quot;,\n    \&quot;amount\&quot; : {\n      \&quot;currencyCode\&quot; : \&quot;USD\&quot;,\n      \&quot;centAmount\&quot; : 1000\n    },\n    \&quot;state\&quot; : \&quot;Pending\&quot;\n  } ]\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer {{ctp_access_token}}</value>
      <webElementGuid>fdcd4580-0fd4-4755-9c9b-d6510b16e166</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>33328006-3fab-48a9-98fb-52035f0a3cd1</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.6.9</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${host}/${project-key}/payments</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.host</defaultValue>
      <description></description>
      <id>2836a94b-f58e-4949-afad-dfdfbdfff7f6</id>
      <masked>false</masked>
      <name>host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.project-key</defaultValue>
      <description></description>
      <id>e55914e7-b69e-4060-a629-c0a028c21b66</id>
      <masked>false</masked>
      <name>project-key</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
